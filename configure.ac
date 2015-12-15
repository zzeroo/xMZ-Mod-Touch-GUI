dnl Process this file with autoconf to produce a configure script.

AC_PREREQ([2.64])

m4_define(xmz_major_version, 0)
m4_define(xmz_minor_version, 4)
m4_define(xmz_micro_version, 2)
m4_define(xmz_version, xmz_major_version.xmz_minor_version.xmz_micro_version)

AC_INIT([xmz],
        [xmz_version],
        [info@gaswarnanlagen.com],
        [xmz],
        [http://gaswarnanlagen.com/xmz])

AC_CONFIG_HEADERS([config.h])
AC_CONFIG_SRCDIR([xmz/xmz.vala])
AC_CONFIG_MACRO_DIR([m4])
AC_CONFIG_AUX_DIR([build-aux])

AC_DEFINE(XMZ_MAJOR_VERSION, xmz_major_version, [xMZ major version])
AC_SUBST(XMZ_MAJOR_VERSION, xmz_major_version)
AC_DEFINE(XMZ_MINOR_VERSION, xmz_minor_version, [xMZ minor version])
AC_SUBST(XMZ_MINOR_VERSION, xmz_minor_version)
AC_DEFINE(XMZ_MICRO_VERSION, xmz_micro_version, [xMZ micro version])
AC_SUBST(XMZ_MICRO_VERSION, xmz_micro_version)

AM_INIT_AUTOMAKE([1.11 tar-ustar dist-xz no-dist-gzip -Wno-portability subdir-objects])
AM_MAINTAINER_MODE([enable])

AM_SILENT_RULES([yes])

# Check for programs
AC_PROG_CC
AC_PROG_INSTALL
AC_PROG_MAKE_SET
AC_PATH_PROG(GZIP, gzip)
AC_PROG_MKDIR_P
AM_PROG_VALAC([0.30.0])

AC_PATH_PROG(GLIB_MKENUMS, glib-mkenums)
AC_PATH_PROG(GLIB_COMPILE_RESOURCES, glib-compile-resources)
AC_PATH_PROGS(GTK_UPDATE_ICON_CACHE, gtk3-update-icon-cache gtk-update-icon-cache gtk-update-icon-cache-3.0)

AC_PATH_PROG(VALADOC, valadoc)

# Initialize libtool
LT_PREREQ([2.2])
LT_INIT(disable-static)

# i18n stuff
IT_PROG_INTLTOOL([0.40.0])

AM_GNU_GETTEXT_VERSION([0.19.6])
AM_GNU_GETTEXT([external])

GETTEXT_PACKAGE=xmz
AC_SUBST(GETTEXT_PACKAGE)

dnl adl_RECURSIVE_EVAL(VALUE, RESULT)
dnl =================================
dnl Interpolate the VALUE in loop until it doesn't change,
dnl and set the result to $RESULT.
dnl WARNING: It's easy to get an infinite loop with some unsane input.
AC_DEFUN([adl_RECURSIVE_EVAL],
[_lcl_receval="$1"
$2=`(test "x$prefix" = xNONE && prefix="$ac_default_prefix"
     test "x$exec_prefix" = xNONE && exec_prefix="${prefix}"
     _lcl_receval_old=''
     while test "[$]_lcl_receval_old" != "[$]_lcl_receval"; do
       _lcl_receval_old="[$]_lcl_receval"
       eval _lcl_receval="\"[$]_lcl_receval\""
     done
     echo "[$]_lcl_receval")`])

adl_RECURSIVE_EVAL("$datadir/xmz", [XMZ_DATADIR])
adl_RECURSIVE_EVAL("$datadir/locale", [XMZ_LOCALEDIR])
adl_RECURSIVE_EVAL("$libdir/xmz", [XMZ_LIBDIR])

AC_DEFINE_UNQUOTED(GETTEXT_PACKAGE,"$GETTEXT_PACKAGE", [Gettext package])
AC_DEFINE_UNQUOTED(XMZ_DATADIR,"$XMZ_DATADIR", [data dir])
AC_DEFINE_UNQUOTED(XMZ_LOCALEDIR,"$XMZ_LOCALEDIR", [locale dir])
AC_DEFINE_UNQUOTED(XMZ_LIBDIR,"$XMZ_LIBDIR", [lib dir])

AC_CHECK_LIB([m], [sinf])

GLIB_REQUIRED_VERSION=2.38
GTK_REQUIRED_VERSION=3.12.0
INTROSPECTION_REQUIRED=0.10.1

PKG_CHECK_MODULES(LIBXMZ, [
                        gthread-2.0 >= $GLIB_REQUIRED_VERSION
                        glib-2.0 >= $GLIB_REQUIRED_VERSION
                        gobject-2.0 >= $GLIB_REQUIRED_VERSION
                        gmodule-2.0 >= $GLIB_REQUIRED_VERSION
                        gio-2.0 >= $GLIB_REQUIRED_VERSION
                        gio-unix-2.0 >= $GLIB_REQUIRED_VERSION
                        gobject-introspection-1.0 >= $INTROSPECTION_REQUIRED
                        gtk+-3.0 >= $GTK_REQUIRED_VERSION
                        gsettings-desktop-schemas
                        gee-0.8
                        sqlite3
                        libmodbus
                        ])

cflags_save="${CFLAGS}"
libs_save="${LIBS}"

CFLAGS="${CFLAGS} ${LIBXMZ_CFLAGS}"
LIBS="${LIBS} ${LIBXMZ_LIBS}"

CFLAGS="${cflags_save}"
LIBS="${libs_save}"


if test "x$USE_MAINTAINER_MODE" = "xyes"; then
  LIBXMZ_CFLAGS="$LIBXMZ_CFLAGS -Wall -Werror -Wuninitialized -Wmissing-declarations"
fi

AC_ARG_ENABLE(debug,
              [AS_HELP_STRING([--disable-debug],
                              [disable debug information [default=yes]])],
              [enable_debug=$enableval], [enable_debug=yes])

XMZ_VALAFLAGS=" \
              --target-glib 2.38 \
              --vapidir \$(top_srcdir)/vapi \
              --vapidir \$(top_builddir)/libxmz"

if test "x$enable_debug" = "xyes"; then
	LIBXMZ_CFLAGS="$LIBXMZ_CFLAGS -g -O0"
	XMZ_VALAFLAGS="$XMZ_VALAFLAGS -g"

	AC_DEFINE([ENABLE_DEBUG],[1],[Whether debugging support is enabled])
fi

AC_SUBST(XMZ_VALAFLAGS)

AC_SUBST(LIBXMZ_CFLAGS)
AC_SUBST(LIBXMZ_LIBS)

PKG_CHECK_MODULES(XMZ, [
                        gsettings-desktop-schemas
                        ])

XMZ_CFLAGS="$LIBXMZ_CFLAGS $XMZ_CFLAGS"
XMZ_LIBS="$LIBXMZ_LIBS $XMZ_LIBS -lm"

AC_SUBST(XMZ_CFLAGS)
AC_SUBST(XMZ_LIBS)

GOBJECT_INTROSPECTION_REQUIRE($INTROSPECTION_REQUIRED)
AC_SUBST(INTROSPECTION_REQUIRED)

INTROSPECTION_GIRDIR=`$PKG_CONFIG \
  --define-variable="prefix=${prefix}" \
  --define-variable="datarootdir=${datarootdir}" \
  --define-variable="datadir=${datadir}" \
  --variable=girdir gobject-introspection-1.0`

INTROSPECTION_TYPELIBDIR=`$PKG_CONFIG \
  --define-variable="prefix=${prefix}" \
  --define-variable="libdir=${libdir}" \
  --variable=typelibdir gobject-introspection-1.0`

GNOME_COMPILE_WARNINGS([maximum])

GLIB_GSETTINGS

GLADE_UI_REQUIRED=3.2

AC_ARG_ENABLE([glade-catalog],
              [AS_HELP_STRING([--enable-glade-catalog[=@<:@no/auto/yes@:>@]],
                              [Install a glade catalog file [default=no]])],
                              [glade_catalog=$enableval],
                              [glade_catalog="no"])

AM_CONDITIONAL([GLADE_CATALOG], [ test "$glade_catalog" = "yes" ])


AC_CONFIG_FILES([
                 Makefile
                 libxmz/libxmz-1.0.pc
                 data/xmz.desktop.in
                 data/com.gaswarnanlagen.xmz.gschema.xml.in
                 po/Makefile.in
                 ])

AC_OUTPUT

echo "

Configuration:

	Source code location:	${srcdir}
	Compiler:		${CC}
	Glade catalog:          ${glade_catalog}
	Debug enabled:          ${enable_debug}
"

