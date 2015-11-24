TESTS_XMZ_PROGS = tests/xmz/test-xmz

TESTS += $(TESTS_XMZ_PROGS)
noinst_PROGRAMS += $(TESTS_XMZ_PROGS)

tests_xmz_test_xmz_CPPFLAGS = \
	-I$(top_srcdir) 			\
	-I$(top_srcdir)/xmz 			\
	-I$(top_srcdir)/tests/support

tests_xmz_test_xmz_VALAFLAGS =		\
	--pkg posix				\
	--pkg xmz-assert 			\
	--pkg gio-2.0				\
	--pkg gtk+-3.0				\
	--pkg gee-0.8				\
	--pkg config				\
	$(XMZ_VALAFLAGS)			\
	--vapidir $(top_srcdir)/tests/support	\
	--vapidir $(top_srcdir)/vapi		\
	--disable-warnings

tests_xmz_test_xmz_CFLAGS =			\
	$(XMZ_DEBUG_FLAGS)			\
	$(XMZ_CFLAGS)				\
	-g					\
	-w

tests_xmz_test_xmz_LDADD =			\
	$(XMZ_LIBS) \
	$(NULL)

tests/xmz/support-%.vala: tests/support/%.vala
	cp "$<" "$@"

tests/xmz/xmz-%.vala: xmz/xmz-%.vala
	cp "$<" "$@"

TESTS_XMZ_TEST_XMZ_COPIED_SOURCES = 			\
	tests/xmz/support-test.vala			\
	tests/xmz/support-main.vala			\
	$(NULL)

tests_xmz_test_xmz_SOURCES =				\
	tests/xmz/main.vala				\
	$(TESTS_XMZ_TEST_XMZ_COPIED_SOURCES)		\
	$(XMZ_PLUGIN_VAPISOURCES) \
	$(NULL)

CLEANFILES += $(TESTS_XMZ_TEST_XMZ_COPIED_SOURCES)



# vi:ts=8:noet
