bin_PROGRAMS += xmz/xmz

xmz_xmz_CPPFLAGS = \
	-I$(top_srcdir) \
	-I$(srcdir) \
	-DDATADIR=\""$(datadir)"\" \
	-DXMZ_DATADIR=\""$(datadir)/xmz"\" \
	-DXMZ_LOCALEDIR=\""$(datadir)/locale"\" \
	$(NULL)

xmz_xmz_CFLAGS = \
	$(XMZ_CFLAGS) \
	$(WARN_CFLAGS) \
	-w \
	$(NULL)


xmz_xmz_VALAFLAGS = \
	--vapidir $(top_srcdir)/vapi \
	$(XMZ_VALAFLAGS) \
	--pkg config \
	--pkg gtk+-3.0 \
	--pkg gio-2.0 \
	--pkg gobject-introspection-1.0 \
	--pkg gee-0.8 \
	--girdir "$(top_builddir)" \
	--vapidir "$(top_srcdir)/vapi" \
	--vapidir "$(top_builddir)/libxmz" \
	--vapidir "$(top_builddir)/libxmz-ext" \
	--gresources "$(top_srcdir)/xmz/resources/xmz-resources.xml" \
	$(NULL)


xmz_xmz_LDADD = \
	libxmz/libxmz-1.0.la \
	libxmz-ext/libxmz-ext-1.0.la \
	$(XMZ_LIBS) \
	$(PACKAGE_LIBS) \
	$(NULL)

xmz_xmz_LDFLAGS = \
	-export-dynamic \
	-no-undefined \
	-export-symbols-regex "^[[^_]].*" \
	$(NULL)


xmz_xmz_VALASOURCES = \
	xmz/sensor/xmz-sensor.vala \
	xmz/xmz.vala \
	xmz/xmz-application.vala \
	xmz/xmz-ui-elements.vala \
	xmz/xmz-notifications.vala \
	xmz/xmz-settings-view.vala \
	xmz/xmz-sensor-view.vala \
	xmz/xmz-simple-notification.vala \
	xmz/xmz-window.vala \
	libxmz-ext/libxmz-ext-1.0.vapi \
	libxmz/libxmz-1.0.vapi \
	$(NULL)

xmz_xmz_SOURCES = \
	$(xmz_xmz_VALASOURCES) \
	xmz/xmz-resources.c \
	$(NULL)

xmz_resources_deps = $(call GRESDEPS,xmz/resources/xmz-resources.xml)
xmz/xmz-resources.c: $(xmz_resources_deps)
	$(GRESGEN)

CLEANFILES += xmz/xmz-resources.c
EXTRA_DIST += $(xmz_resources_deps)


# vi:ts=8:noet
