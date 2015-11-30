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
		$(LIBXMZ_LIBS) \
		$(NULL)

tests_common_valaflags = \
		$(LIBXMZ_VALAFLAGS) \
		--pkg gtk+-3.0 \
		$(NULL)

noinst_PROGRAMS += $(TESTS_DEMO_PROGS)

TESTS_DEMO_PROGS = \
		tests/sensor-list \
		$(NULL)

tests_sensor_list_SOURCES = tests/sensor-list.vala libxmz/libxmz-1.0.vapi
tests_sensor_list_VALAFLAGS = $(tests_common_valaflags)
tests_sensor_list_CFLAGS = $(tests_common_cflags)
tests_sensor_list_CPPFLAGS = $(tests_common_cppflags)
tests_sensor_list_LDADD = libxmz/libxmz-1.0.la $(tests_common_ldadd)

include tests/xmz/Makefile.am

EXTRA_DIST += \
	tests/support/xmz-assert.h \
	tests/support/xmz-assert.vapi \
	$(NULL)


# vi:ts=8:noet
