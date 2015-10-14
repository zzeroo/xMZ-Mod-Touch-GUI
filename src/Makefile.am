# The actual runnable program is set to the SCRIPTS primitive.
# # Prefix bin_ tells where to copy this
bin_PROGRAMS = xmz-mod-touch-gui

BUILT_SOURCES = xmz-mod-touch-gui-resources.c

xmz_mod_touch_gui_SOURCES = \
														main.vala \
														SensorNode.vala \
														SensorModel.vala \
														DatabaseBackend.vala \
														$(BUILT_SOURCES)

#xmz_mod_touch_gui_CFLAGS = $(GTK_CFLAGS)
xmz_mod_touch_gui_CFLAGS = -w \
													 -rdynamic \
													 $(NULL)

xmz_mod_touch_gui_CPPFLAGS = \
														 -DPKGDATADIR=\"$(datadir)/xmz-mod-touch-gui\" \
														 -DLOCALEDIR=\"$(localedir)\" \
														 $(XMZ_MOD_TOUCH_GUI_CFLAGS)

xmz_mod_touch_gui_LDADD = \
													$(GTK_LIBS) \
													$(NULL)

xmz_mod_touch_gui_VALAFLAGS = \
															--pkg posix \
															--pkg gio-2.0 \
															--pkg gtk+-3.0 \
															--pkg gee-0.8 \
															--pkg gmodule-2.0 \
															--pkg sqlite3 \
															--target-glib=$(GLIB_REQUIRED) \
															--gresources=$(abs_top_srcdir)/data/xmz-mod-touch-gui.gresource.xml

xmz-mod-touch-gui-resources.c: $(top_srcdir)/data/xmz-mod-touch-gui.gresource.xml $(shell $(GLIB_COMPILE_RESOURCES) --sourcedir=$(top_srcdir)/data --generate-dependencies $(top_srcdir)/data/xmz-mod-touch-gui.gresource.xml)
	$(AM_V_GEN) $(GLIB_COMPILE_RESOURCES) --target=$@ --sourcedir=$(top_srcdir)/data --generate-source $<

desktopdir = $(datadir)/applications
desktop_DATA = \
							 xmz-mod-touch-gui.desktop

CLEANFILES = \
						 xmz-mod-touch-gui-resources.c

-include $(top_srcdir)/git.mk
