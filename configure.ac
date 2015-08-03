### INITIALIZATION #############################################################
# In this section we do preparations for initializing our build system.

AC_INIT([xMZ-Mod-Touch], [1], [s.mueller@it.kls-glt.de])
AC_CONFIG_SRCDIR([src/main.c])
AM_INIT_AUTOMAKE([-Wall foreign])
AM_SILENT_RULES([yes])

### SHOPPING LIST ##############################################################
# In this section we notify autoconf of what tools we will be using.

AC_PROG_CC
PKG_PROG_PKG_CONFIG

### LIBRARIES ##################################################################
# In this section we use pkg-config to configure the libraries we are using.

PKG_CHECK_MODULES([APP_XMZ], [
	glib-2.0
	gtk+-3.0
])

### OUTPUT #####################################################################
# After checking all this stuff, we must output our results to the rest of the
# build system. That is what this section does.

AC_CONFIG_FILES([
	Makefile
	src/Makefile
	pixmaps/Makefile
])
AC_OUTPUT
