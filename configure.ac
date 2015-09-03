# This file is processed by autoconf to create a configure script
AC_INIT([xMZ-Mod-Touch-GUI], 0.1.1, [s.mueller@it.kls-glt.de])
AM_INIT_AUTOMAKE([1.10 no-define foreign dist-xz no-dist-gzip])

AC_PROG_CC
AM_PROG_VALAC([0.16])
AM_PROG_CC_C_O
PKG_CHECK_MODULES(gtk, gtk+-3.0)

GLIB_GSETTINGS

dnl ###########################################################################
dnl Dependencies
dnl ###########################################################################

GLIB_REQUIRED=2.40.0
GLIB_REQUIRED=2.40.0
GTK_REQUIRED=3.15.0

PKG_CHECK_MODULES(XMZ_MOD_TOUCH_GUI, [
  glib-2.0 >= $GLIB_REQUIRED
  gio-2.0 >= $GLIB_REQUIRED
  gtk+-3.0 >= $GTK_REQUIRED
  gee-0.8
])

PKG_CHECK_MODULES(LIBXMZ, [
  glib-2.0 >= $GLIB_REQUIRED
  gio-2.0 >= $GLIB_REQUIRED
  gee-0.8
])

AC_SUBST([GLIB_REQUIRED])

AC_PATH_PROG([DESKTOP_FILE_VALIDATE], [desktop-file-validate], [/bin/true])

AC_CHECK_LIB([m],[floor])

dnl ###########################################################################
dnl GResources
dnl ###########################################################################

AC_PATH_PROG(GLIB_COMPILE_RESOURCES, glib-compile-resources)
AC_SUBST(GLIB_COMPILE_RESOURCES)

AC_CONFIG_FILES([Makefile
                 src/Makefile
                 data/Makefile])

AC_OUTPUT
