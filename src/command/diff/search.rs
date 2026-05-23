use super::types::{ChangeType, DiffFullscreen, DiffLayout, DiffLine};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SearchMode {
    #[default]
    Inactive,
    InputForward,
}

#[derive(Clone, Debug)]
pub struct SearchMatch {
    pub line_index: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub panel: MatchPanel,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MatchPanel {
    Old,
    New,
}

#[derive(Default, Clone)]
pub struct SearchState {
    pub mode: SearchMode,
    pub query: String,
    pub matches: Vec<SearchMatch>,
    pub current_match: Option<usize>,
}

impl SearchState {
    pub fn start_forward(&mut self) {
        self.mode = SearchMode::InputForward;
        self.query.clear();
        self.matches.clear();
        self.current_match = None;
    }

    pub fn cancel(&mut self) {
        self.mode = SearchMode::Inactive;
        self.query.clear();
        self.matches.clear();
        self.current_match = None;
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current_match = None;
    }

    pub fn confirm(&mut self) {
        self.mode = SearchMode::Inactive;
    }

    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
    }

    pub fn is_active(&self) -> bool {
        self.mode != SearchMode::Inactive
    }

    pub fn has_query(&self) -> bool {
        !self.query.is_empty()
    }

    pub fn update_matches(
        &mut self,
        lines: &[DiffLine],
        fullscreen: DiffFullscreen,
        layout: DiffLayout,
    ) {
        if self.query.is_empty() {
            self.matches.clear();
            self.current_match = None;
            return;
        }

        // Remember current match identity before rebuilding
        let prev_match = self
            .current_match
            .and_then(|idx| self.matches.get(idx))
            .map(|m| (m.line_index, m.start_col, m.end_col, m.panel));

        self.matches.clear();

        let query_lower = self.query.to_lowercase();
        let query_len = self.query.len();

        for (i, line) in lines.iter().enumerate() {
            let scan_old = matches!(layout, DiffLayout::Stack)
                || !matches!(fullscreen, DiffFullscreen::NewOnly);
            let scan_new = matches!(layout, DiffLayout::Stack)
                || !matches!(fullscreen, DiffFullscreen::OldOnly);

            match layout {
                DiffLayout::Split => {
                    if scan_old {
                        self.add_matches_for_text(
                            i,
                            MatchPanel::Old,
                            line.old_line.as_ref().map(|(_, text)| text),
                            &query_lower,
                            query_len,
                        );
                    }
                    if scan_new {
                        self.add_matches_for_text(
                            i,
                            MatchPanel::New,
                            line.new_line.as_ref().map(|(_, text)| text),
                            &query_lower,
                            query_len,
                        );
                    }
                }
                DiffLayout::Stack => match line.change_type {
                    ChangeType::Equal => {
                        if let Some((_, text)) = &line.new_line {
                            self.add_matches_for_text(
                                i,
                                MatchPanel::New,
                                Some(text),
                                &query_lower,
                                query_len,
                            );
                        } else {
                            self.add_matches_for_text(
                                i,
                                MatchPanel::Old,
                                line.old_line.as_ref().map(|(_, text)| text),
                                &query_lower,
                                query_len,
                            );
                        }
                    }
                    ChangeType::Delete => {
                        self.add_matches_for_text(
                            i,
                            MatchPanel::Old,
                            line.old_line.as_ref().map(|(_, text)| text),
                            &query_lower,
                            query_len,
                        );
                    }
                    ChangeType::Insert => {
                        self.add_matches_for_text(
                            i,
                            MatchPanel::New,
                            line.new_line.as_ref().map(|(_, text)| text),
                            &query_lower,
                            query_len,
                        );
                    }
                    ChangeType::Modified => {
                        self.add_matches_for_text(
                            i,
                            MatchPanel::Old,
                            line.old_line.as_ref().map(|(_, text)| text),
                            &query_lower,
                            query_len,
                        );
                        self.add_matches_for_text(
                            i,
                            MatchPanel::New,
                            line.new_line.as_ref().map(|(_, text)| text),
                            &query_lower,
                            query_len,
                        );
                    }
                },
            }
        }

        // Restore current match by identity, or find next visible one
        if let Some((line_idx, start, end, panel)) = prev_match {
            self.current_match = self.matches.iter().position(|m| {
                m.line_index == line_idx
                    && m.start_col == start
                    && m.end_col == end
                    && m.panel == panel
            });

            // If previous match not found (filtered out), find next visible match
            if self.current_match.is_none() && !self.matches.is_empty() {
                // Find first match at or after the previous line
                self.current_match = self
                    .matches
                    .iter()
                    .position(|m| m.line_index >= line_idx)
                    .or(Some(0));
            }
        }
    }

    fn add_matches_for_text(
        &mut self,
        line_index: usize,
        panel: MatchPanel,
        text: Option<&String>,
        query_lower: &str,
        query_len: usize,
    ) {
        let Some(text) = text else {
            return;
        };
        let text_lower = text.to_lowercase();
        let mut start = 0;
        while let Some(pos) = text_lower[start..].find(query_lower) {
            let abs_pos = start + pos;
            self.matches.push(SearchMatch {
                line_index,
                start_col: abs_pos,
                end_col: abs_pos + query_len,
                panel,
            });
            start = abs_pos + 1;
        }
    }

    pub fn find_next(&mut self) -> Option<usize> {
        if self.matches.is_empty() {
            return None;
        }

        let current = self.current_match.unwrap_or(0);
        let next = if current + 1 >= self.matches.len() {
            0 // wrap around
        } else {
            current + 1
        };

        self.current_match = Some(next);
        Some(self.matches[next].line_index)
    }

    pub fn find_prev(&mut self) -> Option<usize> {
        if self.matches.is_empty() {
            return None;
        }

        let current = self.current_match.unwrap_or(0);
        let prev = if current == 0 {
            self.matches.len() - 1 // wrap around
        } else {
            current - 1
        };

        self.current_match = Some(prev);
        Some(self.matches[prev].line_index)
    }

    pub fn jump_to_first_match(&mut self, current_scroll: usize) -> Option<usize> {
        if self.matches.is_empty() {
            return None;
        }

        let idx = self
            .matches
            .iter()
            .position(|m| m.line_index >= current_scroll)
            .unwrap_or(0);
        self.current_match = Some(idx);
        Some(self.matches[idx].line_index)
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    pub fn current_match_index(&self) -> Option<usize> {
        self.current_match
    }

    pub fn get_matches_for_line(
        &self,
        line_index: usize,
        panel: MatchPanel,
    ) -> Vec<(usize, usize, bool)> {
        self.matches
            .iter()
            .enumerate()
            .filter(|(_, m)| m.line_index == line_index && m.panel == panel)
            .map(|(idx, m)| {
                let is_current = self.current_match == Some(idx);
                (m.start_col, m.end_col, is_current)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::diff::types::InlineSegment;

    fn diff_line(old: Option<&str>, new: Option<&str>, change_type: ChangeType) -> DiffLine {
        DiffLine {
            old_line: old.map(|text| (1, text.to_string())),
            new_line: new.map(|text| (1, text.to_string())),
            change_type,
            old_segments: None::<Vec<InlineSegment>>,
            new_segments: None::<Vec<InlineSegment>>,
        }
    }

    #[test]
    fn stack_search_includes_old_and_new_modified_text() {
        let lines = vec![diff_line(
            Some("old needle"),
            Some("new needle"),
            ChangeType::Modified,
        )];
        let mut state = SearchState {
            query: "needle".to_string(),
            ..SearchState::default()
        };

        state.update_matches(&lines, DiffFullscreen::NewOnly, DiffLayout::Stack);

        assert_eq!(state.matches.len(), 2);
        assert_eq!(state.matches[0].panel, MatchPanel::Old);
        assert_eq!(state.matches[1].panel, MatchPanel::New);
    }
}
