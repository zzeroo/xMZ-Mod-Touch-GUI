tests_common_cppflags = \
		-I$(top_srcdir) \
		-I$(top_srcdir)/src \
		$(NULL)

tests_common_clags = \
		-g \
		$(XMZ_DEBUG_FLAGS) \
		$(XMZ_CFLAGS) \
		$(NULL)

tests_common_ldadd = \
		$(NULL)

tests_common_valaflags = \
		$(NULL)

noinst_PROGRAMS += $(TESTS_DEMO_PROGS)

TESTS_DEMO_PROGS = \
		tests/sensor-view \
		$(NULL)

tests_sensor_view_SOURCES = tests/sensor-view.vala
tests_sensor_view_VALAFLAGS = $(tests_common_valaflags)
tests_sensor_view_CFLAGS = $(tests_common_clags)
tests_sensor_view_CPPFLAGS = $(tests_common_cppflags)
tests_sensor_view_LDADD = $(tests_common_ldadd)




# vi:ts=8:noet
