include $(top_srcdir)/Makefile.decl

tests_common_cppflags = \
		-I$(top_srcdir) \
		-I$(top_srcdir)/xmz \
		$(NULL)

tests_common_cflags = \
		-g \
		$(XMZ_DEBUG_FLAGS) \
		$(XMZ_CFLAGS) \
		-w \
		$(NULL)

tests_common_ldadd = \
		$(XMZ_PLUGIN_LIBS) \
		$(NULL)

tests_common_valaflags = \
		$(XMZ_PLUGIN_VALAFLAGS) \
		$(NULL)

noinst_PROGRAMS += $(TESTS_DEMO_PROGS)

TESTS_DEMO_PROGS = \
		$(NULL)

include tests/xmz/Makefile.am

EXTRA_DIST += \
	tests/support/xmz-assert.h \
	tests/support/xmz-assert.vapi \
	$(NULL)


# vi:ts=8:noet
