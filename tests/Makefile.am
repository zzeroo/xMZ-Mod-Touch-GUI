include $(top_srcdir)/Makefile.decl

tests_common_cppflags = \
		-I$(top_srcdir) \
		-I$(top_srcdir)/xmz \
		-I$(top_srcdir)/libxmz \
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
		tests/sensor-view \
		$(NULL)

tests_sensor_view_SOURCES = tests/sensor-view.vala $(XMZ_PLUGIN_VAPISOURCES)
tests_sensor_view_VALAFLAGS = $(tests_common_valaflags)
tests_sensor_view_CFLAGS = $(tests_common_cflags)
tests_sensor_view_CPPFLAGS = $(tests_common_cppflags)
tests_sensor_view_LDADD = libxmz/libxmz-1.0.la $(tests_common_ldadd)

include tests/libxmz/Makefile.am
include tests/xmz/Makefile.am

EXTRA_DIST += \
	tests/support/xmz-assert.h \
	tests/support/xmz-assert.vapi \
	$(NULL)


# vi:ts=8:noet
