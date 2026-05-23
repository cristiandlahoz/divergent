BINDIR ?= $(HOME)/.cargo/bin
BIN := divergent
TARGET := target/release/$(BIN)

.PHONY: build install uninstall check path-check

build:
	cargo build --release

install: build
	install -d "$(BINDIR)"
	install "$(TARGET)" "$(BINDIR)/$(BIN)"
	@printf 'installed %s to %s\n' "$(BIN)" "$(BINDIR)/$(BIN)"

uninstall:
	rm -f "$(BINDIR)/$(BIN)"
	@printf 'removed %s\n' "$(BINDIR)/$(BIN)"

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	cargo test --doc

path-check:
	@case ":$$PATH:" in \
		*:"$(BINDIR)":*) printf '%s is on PATH\n' "$(BINDIR)" ;; \
		*) printf 'warning: %s is not on PATH\n' "$(BINDIR)" ;; \
	esac
