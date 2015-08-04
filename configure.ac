#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.69])
AC_INIT([XMZ], [1], [s.mueller@it.kls-glt.de])
AM_INIT_AUTOMAKE([-Wall -Werror foreign])
AC_CONFIG_SRCDIR([src/main.c])
AC_CONFIG_HEADERS([config.h])

# Checks for programs.
AC_PROG_CC

# Check for GLIB_COMPILE_RESOURCES

AC_PATH_PROG(GLIB_COMPILE_RESOURCES,[glib-compile-resources],no)

if test x$GLIB_COMPILE_RESOURCES = xno ; then
AC_MSG_ERROR([glib-compile-resources cannot be found in PATH.
              It is usually included with GTK+3.0 dev. libraries.
              Please install this tool first.
])
fi

AC_SUBST(GLIB_COMPILE_RESOURCES)

# Checks for libraries.
AM_PATH_GTK_3_0
#PKG_CHECK_MODULES([GLIB], [glib-2.0 > 2.19.0])
#AC_SUBST(GLIB_CFLAGS)

# Checks for header files.

# Checks for typedefs, structures, and compiler characteristics.

# Checks for library functions.

AC_OUTPUT(
          Makefile
          src/Makefile
          )
