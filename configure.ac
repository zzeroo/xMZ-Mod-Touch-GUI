dnl Process this file with autoconf to produce a configure script.

AC_PREREQ([2.64])

m4_define(xmz_major_version, 0)
m4_define(xmz_minor_version, 5)
m4_define(xmz_micro_version, 1)
m4_define(xmz_version, xmz_major_version.xmz_minor_version.xmz_micro_version)

AC_INIT([xmz],
        [xmz_version],
        [info@gaswarnanlagen.com],
        [xmz],
        [http://gaswarnanlagen.com/xmz])

AC_CONFIG_HEADERS([config.h])
AC_CONFIG_SRCDIR([src/xmz.vala])
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
AC_PATH_PROGS(GTK_UPDATE_ICON_CACHE, gtk3-update-icon-cache gtk-update-icon-cache)

AC_PATH_PROG(VALADOC, valadoc)

AC_CHECK_LIB([m], [sinf])

GLIB_REQUIRED_VERSION=2.38
GTK_REQUIRED_VERSION=3.12.0
INTROSPECTION_REQUIRED=0.10.1

PKG_CHECK_MODULES(XMZ, [
                        gthread-2.0 >= $GLIB_REQUIRED_VERSION
                        glib-2.0 >= $GLIB_REQUIRED_VERSION
                        gobject-2.0 >= $GLIB_REQUIRED_VERSION
                        gmodule-2.0 >= $GLIB_REQUIRED_VERSION
                        gio-2.0 >= $GLIB_REQUIRED_VERSION
                        gio-unix-2.0 >= $GLIB_REQUIRED_VERSION
                        gobject-introspection-1.0 >= $INTROSPECTION_REQUIRED
                        gtk+-3.0 >= $GTK_REQUIRED_VERSION
                        gsettings-desktop-schemas
                        gee-1.0
                        sqlite3
                        ])

XMZ_VALAFLAGS="--target-glib 2.38"

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


AC_CONFIG_FILES([
                 Makefile
                 src/Makefile
                 ])

AC_OUTPUT
