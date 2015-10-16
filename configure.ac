# This file is processed by autoconf to create a configure script
AC_INIT([xMZ-Mod-Touch-GUI], 0.1.1, [s.mueller@it.kls-glt.de])
AC_CONFIG_SRCDIR([Makefile.am])
AM_INIT_AUTOMAKE([1.10 no-define foreign dist-xz no-dist-gzip])
m4_ifdef([AM_SILENT_RULES], [AM_SILENT_RULES([yes])])

# Checks for programs.
AC_PROG_CC
AM_PROG_CC_C_O

LIBGEE_LT_VERSION="7:1:5"
AC_SUBST(LIBGEE_LT_VERSION)

# FIXME: Find how libgee handle AM_PROG_VALAC
AM_PROG_VALAC([0.16])

PKG_CHECK_MODULES(GTK, gtk+-3.0)
PKG_CHECK_MODULES(SQLITE, sqlite3)

GLIB_GSETTINGS

dnl ###########################################################################
dnl Dependencies
dnl ###########################################################################

GLIB_REQUIRED=2.40.0
GLIB_REQUIRED=2.40.0
GTK_REQUIRED=3.15.0

VALA_ADD_CHECKFILE([tests/tests_vala.stamp])
VALA_ADD_VALAFLAGS(--target-glib=$GLIB_REQUIRED)
VALA_ADD_VALAFLAGS(--hide-internal)

AC_ARG_ENABLE(doc, AS_HELP_STRING([--enable-doc], [Enable documentation generation]), enable_doc=$enableval, enable_doc=no)
AM_CONDITIONAL(ENABLE_DOC, test x$enable_doc = xyes)

AS_IF([test "x$enable_doc" != xno],
      [AC_PATH_PROG(VALADOC, valadoc, :)
       AC_SUBST(VALADOC)
       AS_IF([test "$VALADOC" = :],
             [AC_MSG_ERROR([valadoc not found])])])

AC_ARG_ENABLE(coverage,
	AS_HELP_STRING([--enable-coverage],
	[compile with coverage profiling and debug (gcc only)]),
	enable_coverage=$enableval,enable_coverage=no)
AM_CONDITIONAL(ENABLE_COVERAGE, test x$enable_coverage != xno)
AS_IF([test "x$enable_coverage" != xno],
	[COVERAGE_CFLAGS="-fprofile-arcs -ftest-coverage"
	 COVERAGE_VALAFLAGS="-g"
	 COVERAGE_LIBS="-lgcov"
	 AC_SUBST(COVERAGE_CFLAGS)
	 AC_SUBST(COVERAGE_VALAFLAGS)
	 AC_SUBST(COVERAGE_LIBS)
	 AC_PATH_PROG(LCOV, lcov, :)
	 AC_SUBST(LCOV)
	 AS_IF([test "$LCOV" = :],
		[AC_MSG_ERROR([lcov is necessary for testing code coverage (http://ltp.sourceforge.net/coverage/lcov.php)])])])

AC_ARG_ENABLE(benchmark, AS_HELP_STRING([--enable-benchmark], [Enable benchmark]), enable_benchmark=$enableval, enable_benchmark=no)
AM_CONDITIONAL(ENABLE_BENCHMARK, test x$enable_benchmark = xyes)
AS_IF([test x$enable_benchmark = xyes],
      [VALA_ADD_CHECKFILE([benchmark/benchmarks_vala.stamp])])

PKG_CHECK_MODULES(GLIB, glib-2.0 >= $GLIB_REQUIRED gobject-2.0 >= $GLIB_REQUIRED gio-2.0 >= $GLIB_REQUIRED)
AC_SUBST(GLIB_CFLAGS)
AC_SUBST(GLIB_LIBS)

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

PKG_CHECK_MODULES(GLIB, glib-2.0 >= $GLIB_REQUIRED gobject-2.0 >= $GLIB_REQUIRED gio-2.0 >= $GLIB_REQUIRED)
AC_SUBST(GLIB_CFLAGS)
AC_SUBST(GLIB_LIBS)

VALA_CHECK([0.25.1])

AC_OUTPUT([Makefile
           src/Makefile
           data/Makefile
           tests/Makefile])

