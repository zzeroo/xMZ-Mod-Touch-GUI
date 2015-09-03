# The actual runnable program is set to the SCRIPTS primitive.
# # Prefix bin_ tells where to copy this
bin_PROGRAMS = xmz-mod-touch-gui
xmz_mod_touch_gui_CFLAGS = $(gtk_CFLAGS)
xmz_mod_touch_gui_LDADD = $(gtk_LIBS)
xmz_mod_touch_gui_VALAFLAGS = --pkg gtk+-3.0
xmz_mod_touch_gui_SOURCES = xmz-mod-touch-gui.vala

desktopdir = $(datadir)/applications
desktop_DATA = \
	xmz-mod-touch-gui.desktop

-include $(top_srcdir)/git.mk
