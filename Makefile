CC ?= gcc
PKGCONFIG = $(shell which pkg-config)
CFLAGS = $(shell $(PKGCONFIG) --cflags gtk+-3.0)
LIBS = $(shell $(PKGCONFIG) --libs gtk+-3.0)

SRC = main.c xmz-mod-touch_app.c xmz-mod-touch_appwin.c

OBJS = $(SRC:.c=.o)

all: xmz-mod-touch_app

%.o: %.c
	$(CC) -c -o $(@F) $(CFLAGS) $<

xmz-mod-touch_app: $(OBJS)
	$(CC) -o $(@F) $(LIBS) $(OBJS)

clean:
	rm -f $(OBJS)
	rm -f xmz-mod-touch_app
