# This file is processed by autoconf to create a configure script
AC_INIT([xMZ-Mod-Touch-GUI], 0.1.1, [s.mueller@it.kls-glt.de])
AM_INIT_AUTOMAKE([1.10 no-define foreign dist-xz no-dist-gzip])
AC_PROG_CC
AM_PROG_VALAC([0.16])
PKG_CHECK_MODULES(gtk, gtk+-3.0)

dnl ###########################################################################
dnl GResources
dnl ###########################################################################

AC_PATH_PROG(GLIB_COMPILE_RESOURCES, glib-compile-resources)
AC_SUBST(GLIB_COMPILE_RESOURCES)

AC_CONFIG_FILES([Makefile
                 src/Makefile
                 data/Makefile])

AC_OUTPUT
