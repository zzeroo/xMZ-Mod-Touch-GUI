bin_PROGRAMS=xmz_mod_touch
xmz_mod_touch_SOURCES = 				\
	main.c 					\
	xmzapp.c xmzapp.h 		\
	xmzappwin.c xmzappwin.h 	\
	resources.c

BUILT_SOURCES = resources.c

resources.c: xmzapp.gresource.xml window.ui
	$(AM_V_GEN) $(GLIB_COMPILE_RESOURCES) $(srcdir)/xmzapp.gresource.xml \
		--target=$@ --sourcedir=$(srcdir) --generate-source

EXTRA_DIST = 					\
	window.ui 				\
	xmzapp.gresource.xml

AM_CPPFLAGS = @GTK_CFLAGS@
LDADD = @GTK_LIBS@
