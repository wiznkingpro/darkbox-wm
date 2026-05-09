PREFIX = /usr/local
MANPREFIX = $(PREFIX)/share/man
VERSION = 0.1

CFLAGS = -DVERSION=\"$(VERSION)\" -D_DEFAULT_SOURCE \
         $(shell pkg-config --cflags wayland-server wlroots) \
         -Wall -Wextra -Wno-unused-parameter -Wno-unused-function \
         -Wno-missing-field-initializers

LDLIBS = $(shell pkg-config --libs wlroots wayland-server xkbcommon) -lrt

all: dwl

dwl: dwl.o util.o client.h config.h
	$(CC) dwl.o util.o $(CFLAGS) $(LDLIBS) -o $@

dwl.o: dwl.c config.h client.h protocols/wlr-layer-shell-unstable-v1.h
	$(CC) -c dwl.c $(CFLAGS) -o $@

util.o: util.c util.h
	$(CC) -c util.c $(CFLAGS) -o $@

protocols/wlr-layer-shell-unstable-v1.h:
	wayland-scanner server-header \
		protocols/wlr-layer-shell-unstable-v1.xml \
		protocols/wlr-layer-shell-unstable-v1.h

config.h:
	cp config.def.h config.h

clean:
	rm -f dwl dwl.o util.o config.h

install: all
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f dwl $(DESTDIR)$(PREFIX)/bin
	chmod 755 $(DESTDIR)$(PREFIX)/bin/dwl
	mkdir -p $(DESTDIR)$(HOME)/.local/bin
	cp scripts/* $(DESTDIR)$(HOME)/.local/bin/
	chmod +x $(DESTDIR)$(HOME)/.local/bin/*
	mkdir -p $(DESTDIR)$(HOME)/.config/foot
	mkdir -p $(DESTDIR)$(HOME)/.config/waybar
	cp config/foot/foot.ini $(DESTDIR)$(HOME)/.config/foot/
	cp config/waybar/* $(DESTDIR)$(HOME)/.config/waybar/

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/dwl
	rm -f $(DESTDIR)$(HOME)/.local/bin/statusbar.sh
	rm -f $(DESTDIR)$(HOME)/.local/bin/autostart.sh

.PHONY: all clean install uninstall