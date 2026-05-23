mod diff_view;
mod footer;
pub mod modal;
mod sidebar;

pub use diff_view::{render_diff, render_empty_state, DiffRenderInput};
pub use footer::truncate_path;
pub use modal::{
    FilePickerItem, FileStatus as ModalFileStatus, KeyBind, KeyBindSection, Modal, ModalContent,
    ModalResult,
};
