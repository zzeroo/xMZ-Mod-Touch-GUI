bin_PROGRAMS += xmz/xmz

xmz_xmz_CPPFLAGS = \
	-I$(top_srcdir) \
	-I$(srcdir) \
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
	--pkg gee-1.0 \
	--girdir "$(top_builddir)" \
	--vapidir "$(top_srcdir)/vapi" \
	--gresources "$(top_srcdir)/xmz/resources/xmz-resources.xml" \
	$(NULL)


xmz_xmz_LDADD = \
	$(XMZ_LIBS) \
	$(NULL)

xmz_xmz_LDFLAGS = \
	$(NULL)


xmz_xmz_VALASOURCES = \
	xmz/xmz.vala \
	$(NULL)

xmz_xmz_SOURCES = \
	$(xmz_xmz_VALASOURCES) \
	$(NULL)
