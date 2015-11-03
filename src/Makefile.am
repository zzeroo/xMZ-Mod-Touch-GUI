bin_PROGRAMS += src/xmz

src_xmz_CPPFLAGS = \
	-I$(top_srcdir) \
	-I$(srcdir) \
	$(NULL)

src_xmz_CFLAGS = \
	$(XMZ_CFLAGS) \
	$(WARN_CFLAGS) \
	-w \
	$(NULL)


src_xmz_VALAFLAGS = \
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


src_xmz_LDADD = \
	$(XMZ_LIBS) \
	$(NULL)

src_xmz_LDFLAGS = \
	$(NULL)


src_xmz_VALASOURCES = \
	src/xmz.vala \
	$(NULL)

src_xmz_SOURCES = \
	$(src_xmz_VALASOURCES) \
	$(NULL)
