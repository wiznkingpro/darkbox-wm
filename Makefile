PREFIX = /usr/local
BIN = darkbox-wm

all:
	cargo build --release
	@echo "Binary ready: target/release/$(BIN)"

install:
	sudo cp target/release/$(BIN) $(PREFIX)/bin/
	sudo chmod 755 $(PREFIX)/bin/$(BIN)
	@echo "DarkBox WM installed to $(PREFIX)/bin"

uninstall:
	sudo rm -f $(PREFIX)/bin/$(BIN)

clean:
	cargo clean