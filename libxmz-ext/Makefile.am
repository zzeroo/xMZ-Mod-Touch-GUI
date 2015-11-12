libxmz_extexecdir = $(libdir)
libxmz_extexec_LTLIBRARIES = libxmz-ext/libxmz-ext-1.0.la

libxmz_ext_libxmz_ext_1_0_la_CPPFLAGS = \
	-I$(top_srcdir) \
	-I$(srcdir) \
	-DDATADIR=\""$(datadir)"\" \
	-DLIBDIR=\""$(libdir)"\" \
	$(NULL)

libxmz_ext_libxmz_ext_1_0_la_CFLAGS = \
	$(LIBXMZ_EXT_CFLAGS) \
	$(WARN_CFLAGS) \
	-w \
	$(NULL)

libxmz_ext_libxmz_ext_1_0_la_VALAFLAGS = \
	--pkg gtk+-3.0 \
	--pkg gio-2.0 \
	--pkg gee-1.0 \
	$(XMZ_VALAFLAGS) \
	--basedir $(top_srcdir)		\
	--vapidir $(top_srcdir)/vapi	\
	--gir XMZExt-1.0.gir		\
	--vapi libxmz-ext/libxmz-ext-1.0.vapi	\
	--girdir $(top_builddir)/libxmz \
	--includedir libxmz-ext		\
	--header libxmz-ext/libxmz-ext.h	\
	--library libxmz-ext/libxmz-ext-1.0	\
	$(NULL)

libxmz_ext_libxmz_ext_1_0_la_LDFLAGS = \
	-export-dynamic \
	-no-undefined \
	-export-symbols-regex "^[^_].*" \
	$(NULL)

libxmz_ext_libxmz_ext_1_0_la_LIBADD = \
	libxmz/libxmz-1.0.la \
	$(LIBXMZ_EXT_LIBS) \
	$(NULL)

libxmz_ext_libxmz_ext_1_0_la_VALASOURCES = \
	libxmz-ext/xmz-ext-activity.vala \
	libxmz-ext/xmz-ext-application.vala \
	libxmz-ext/xmz-ext-notification.vala \
	libxmz-ext/xmz-ext-notifications.vala \
	libxmz-ext/xmz-ext-ui-element.vala \
	libxmz-ext/xmz-ext-ui.vala \
	libxmz/libxmz-1.0.vapi \
	$(NULL)

libxmz_ext_libxmz_ext_1_0_la_SOURCES = \
	$(libxmz_ext_libxmz_ext_1_0_la_VALASOURCES)	\
	libxmz-ext/xmz-ext-resources.c \
	$(NULL)

libxmz_ext_libxmz_ext_1_0_la_headerdir = $(prefix)/include/libxmz-ext-1.0/libxmz-ext
libxmz_ext_libxmz_ext_1_0_la_header_DATA = libxmz-ext/libxmz-ext.h

libxmz_ext_libxmz_ext_1_0_la_girdir = $(datadir)/gir-1.0
libxmz_ext_libxmz_ext_1_0_la_gir_DATA = XMZExt-1.0.gir

libxmz_ext_libxmz_ext_1_0_la_typelibdir = $(libdir)/girepository-1.0
libxmz_ext_libxmz_ext_1_0_la_typelib_DATA = XMZExt-1.0.typelib

libxmz_ext_libxmz_ext_1_0_la_vapidir = $(prefix)/share/vala/vapi
libxmz_ext_libxmz_ext_1_0_la_vapi_DATA = libxmz-ext/libxmz-ext-1.0.vapi

libxmz_ext_libxmz_ext_1_0_la_pkgconfigdir = $(libdir)/pkgconfig
libxmz_ext_libxmz_ext_1_0_la_pkgconfig_DATA = libxmz-ext/libxmz-ext-1.0.pc

libxmz_ext_resources_deps = $(call GRESDEPS,libxmz-ext/resources/resources.xml)
libxmz-ext/xmz-ext-resources.c: $(libxmz_ext_resources_deps)
	$(GRESGEN)

GITIGNOREFILES +=					\
	libxmz-ext/xmz-ext-resources.c \
	$(libxmz_ext_libxmz_ext_1_0_la_gir_DATA)		\
	libxmz-ext/libxmz-ext.h

CLEANFILES +=						\
	$(libxmz_ext_libxmz_ext_1_0_la_typelib_DATA)		\
	$(NULL)

EXTRA_DIST +=						\
	$(libxmz_ext_libxmz_ext_1_0_la_pkgconfig_DATA)	\
	$(libxmz_ext_libxmz_ext_1_0_la_gir_DATA)		\
	$(libxmz_ext_resources_deps)

# This is a hack to make parallel install work. libtool needs to relink at
# install (on some platforms) which requires dependency libraries to be
# already installed. automake does not generate the appropriate dependency
# so we do it here automatically. As an additional bonus, we have to trick
# automake using a variable indirection for the rule because it would otherwise
# omit the default install rule.
install_libxmz_extexecLTLIBRARIES = install-libxmz_extexecLTLIBRARIES
$(install_libxmz_extexecLTLIBRARIES): install-libxmzexecLTLIBRARIES

# vi:ts=8:noet
