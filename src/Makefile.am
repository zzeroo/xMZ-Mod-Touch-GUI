bin_PROGRAMS += src/xmz

src_xmz_CPPFLAGS = \
									 -I$(top_srcdir) \
									 -I$(srcdir)

src_xmz_CFLAGS = \
								 $(XMZ_CFLAGS)


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
										--gresources "$(top_srcdir)/src/resources/xmz-resources.xml"


src_xmz_LDADD = \
								$(XMZ_LIBS)

src_xmz_LDFLAGS = \
									-export-dynamic \
									-no-undefined \
									-export-symbols-regex "^[[^_]].*"


src_xmz_VALASOURCES = \
											main.vala

src_xmz_SOURCES = \
									$(src_xmz_VALASOURCES)



-include $(top_srcdir)/git.mk
