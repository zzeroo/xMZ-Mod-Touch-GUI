libxmzexecdir = $(libdir)
libxmzexec_LTLIBRARIES = libxmz/libxmz-1.0.la

libxmz_libxmz_1_0_la_CPPFLAGS = \
	-I$(top_srcdir) \
	-I$(srcdir) \
	-DDATADIR=\""$(datadir)"\" \
	-DLIBDIR=\""$(libdir)"\" \
	-DGETTEXT_PACKAGE=\"$(GETTEXT_PACKAGE)\" \
	$(NULL)

libxmz_libxmz_1_0_la_CFLAGS = \
	$(LIBXMZ_CFLAGS) \
	$(WARN_CFLAGS) \
	-w \
	$(NULL)

libxmz_libxmz_1_0_la_LDFLAGS = \
	-export-dynamic \
	-no-undefined \
	-export-symbols-regex "^[^_].*" \
	$(NULL)

libxmz_libxmz_1_0_la_LIBADD = \
	$(LIBXMZ_LIBS) \
	$(NULL)

libxmz_libxmz_1_0_la_VALAFLAGS = \
	--pkg gtk+-3.0 \
	--pkg gio-2.0 \
	--pkg gee-1.0 \
	$(XMZ_VALAFLAGS) \
	--vapidir $(top_srcdir)/vapi	\
	--includedir libxmz		\
	--basedir $(top_srcdir)		\
	--gir XMZ-1.0.gir		\
	--vapi libxmz/libxmz-1.0.vapi	\
	--library libxmz/libxmz-1.0	\
	--header libxmz/libxmz.h	\
	--gresources "$(top_srcdir)/libxmz/resources/resources.xml" \
	$(NULL)

libxmz_libxmz_1_0_la_VALASOURCES = \
	libxmz/xmz-init.vala \
	libxmz/xmz-resource.vala \
	libxmz/xmz-sensor-list-box.vala \
	$(NULL)

libxmz_libxmz_1_0_la_SOURCES = \
	$(libxmz_libxmz_1_0_la_VALASOURCES)	\
	libxmz/xmz-resources.c \
	$(NULL)

libxmz_libxmz_1_0_la_headerdir = $(prefix)/include/libxmz-1.0/libxmz
libxmz_libxmz_1_0_la_header_HEADERS = libxmz/libxmz.h

libxmz_libxmz_1_0_la_vapidir = $(prefix)/share/vala/vapi
libxmz_libxmz_1_0_la_vapi_DATA = libxmz/libxmz-1.0.vapi

libxmz_libxmz_1_0_la_girdir = $(datadir)/gir-1.0
libxmz_libxmz_1_0_la_gir_DATA = XMZ-1.0.gir

libxmz_libxmz_1_0_la_typelibdir = $(libdir)/girepository-1.0
libxmz_libxmz_1_0_la_typelib_DATA = XMZ-1.0.typelib

libxmz_libxmz_1_0_la_pkgconfigdir = $(libdir)/pkgconfig
libxmz_libxmz_1_0_la_pkgconfig_DATA = libxmz/libxmz-1.0.pc

libxmz_resources_deps = $(call GRESDEPS,libxmz/resources/resources.xml)
libxmz/xmz-resources.c: $(libxmz_resources_deps)
	$(GRESGEN)

GITIGNOREFILES +=					\
	$(libxmz_libxmz_1_0_la_gir_DATA)		\
	libxmz/libxmz.h

CLEANFILES +=						\
	$(libxmz_libxmz_1_0_la_typelib_DATA)		\
	libxmz/xmz-resources.c

EXTRA_DIST +=						\
	$(libxmz_libxmz_1_0_la_pkgconfig_DATA)	\
	$(libxmz_libxmz_1_0_la_gir_DATA)		\
	$(libxmz_resources_deps)

# vi:ts=8:noet
