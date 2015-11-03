bin_PROGRAMS = xmz

xmz_CPPFLAGS = \
	-I$(top_srcdir) \
	-I$(srcdir) \
	$(NULL)

xmz_CFLAGS = \
	$(XMZ_CFLAGS) \
	$(WARN_CFLAGS) \
	-w \
	$(NULL)


xmz_VALAFLAGS = \
	--vapidir $(top_srcdir)/vapi \
	$(XMZ_VALAFLAGS) \
	--pkg config \
	--pkg gtk+-3.0 \
	--pkg gio-2.0 \
	--pkg gobject-introspection-1.0 \
	--pkg gee-1.0 \
	--girdir "$(top_builddir)" \
	--vapidir "$(top_srcdir)/vapi" \
	--gresources "$(top_srcdir)/src/resources/xmz-resources.xml" \
	$(NULL)


xmz_LDADD = \
	$(XMZ_LIBS) \
	$(NULL)

xmz_LDFLAGS = \
	$(NULL)


xmz_VALASOURCES = \
	xmz.vala \
	$(NULL)

xmz_SOURCES = \
	$(xmz_VALASOURCES) \
	$(NULL)
