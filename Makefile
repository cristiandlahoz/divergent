BINDIR ?= $(HOME)/.cargo/bin
BIN := divergent
RELEASE_BIN := target/release/$(BIN)

.PHONY: build install uninstall check path-check git-install git-uninstall

build:
	cargo build --release

install: build
	install -d "$(BINDIR)"
	install "$(RELEASE_BIN)" "$(BINDIR)/$(BIN)"

uninstall:
	rm -f "$(BINDIR)/$(BIN)"

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

path-check:
	@case ":$$PATH:" in \
		*:"$(BINDIR)":*) echo "$(BINDIR) is on PATH" ;; \
		*) echo "warning: $(BINDIR) is not on PATH" >&2 ;; \
	esac

git-install: install
	"$(BINDIR)/$(BIN)" git install --force --binary "$(BINDIR)/$(BIN)"

git-uninstall:
	"$(BINDIR)/$(BIN)" git uninstall
