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
		libxmz/libxmz-1.0.la \
		$(NULL)

tests_common_valaflags = \
		$(LIBXMZ_VALAFLAGS) \
		--pkg config \
		--pkg gtk+-3.0 \
		--pkg libmodbus \
		--vapidir $(top_srcdir)/vapi \
		$(NULL)

noinst_PROGRAMS += $(TESTS_DEMO_PROGS)

TESTS_DEMO_PROGS = \
		tests/sensor-list \
		tests/modbus-server \
		tests/modbus-client \
		$(NULL)

tests_sensor_list_SOURCES = tests/sensor-list.vala libxmz/libxmz-1.0.vapi
tests_sensor_list_VALAFLAGS = $(tests_common_valaflags)
tests_sensor_list_CFLAGS = $(tests_common_cflags)
tests_sensor_list_CPPFLAGS = $(tests_common_cppflags)
tests_sensor_list_LDADD = $(tests_common_ldadd)

tests_modbus_server_SOURCES = tests/modbus-server.vala libxmz/libxmz-1.0.vapi
tests_modbus_server_VALAFLAGS = $(tests_common_valaflags)
tests_modbus_server_CFLAGS = $(tests_common_cflags)
tests_modbus_server_CPPFLAGS = $(tests_common_cppflags)
tests_modbus_server_LDADD = $(tests_common_ldadd)

tests_modbus_client_SOURCES = tests/modbus-client.vala libxmz/libxmz-1.0.vapi
tests_modbus_client_VALAFLAGS = $(tests_common_valaflags)
tests_modbus_client_CFLAGS = $(tests_common_cflags)
tests_modbus_client_CPPFLAGS = $(tests_common_cppflags)
tests_modbus_client_LDADD = $(tests_common_ldadd)

include tests/xmz/Makefile.am

EXTRA_DIST += \
	tests/support/xmz-assert.h \
	tests/support/xmz-assert.vapi \
	$(NULL)


# vi:ts=8:noet
