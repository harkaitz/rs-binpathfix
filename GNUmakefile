PROJECT=rs-binpathfix
VERSION=1.0.0
PREFIX=/usr/local
PROGS_RUST=debug/binpathfix
all:
clean:
install:

## -- BLOCK:license --
install: install-license
install-license: 
	install -D -t $(DESTDIR)$(PREFIX)/share/doc/$(PROJECT) LICENSE
## -- BLOCK:license --
## -- BLOCK:rust --
all: all-rs
all-rs:
	cargo build
install: install-rs
install-rs:
	install -D -t $(DESTDIR)$(PREFIX)/bin $${CARGO_TARGET_DIR:-target}/$(PROGS_RUST)
## -- BLOCK:rust --
