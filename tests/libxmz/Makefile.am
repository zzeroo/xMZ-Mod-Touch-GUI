TESTS_LIBXMZ_PROGS = tests/libxmz/test-libxmz

TESTS += $(TESTS_LIBXMZ_PROGS)
noinst_PROGRAMS += $(TESTS_LIBXMZ_PROGS)

tests_libxmz_test_libxmz_CPPFLAGS = 		\
	-I$(top_srcdir) 			\
	-I$(top_srcdir)/xmz 			\
	-I$(top_srcdir)/libxmz			\
	-I$(top_srcdir)/tests/support

tests_libxmz_test_libxmz_VALAFLAGS =		\
	--pkg posix				\
	--pkg xmz-assert 			\
	--pkg gio-2.0				\
	--pkg gtk+-3.0				\
	--pkg gee-1.0				\
	--pkg config				\
	$(XMZ_VALAFLAGS)			\
	--vapidir $(top_srcdir)/tests/support	\
	--vapidir $(top_srcdir)/vapi		\
	--disable-warnings

tests_libxmz_test_libxmz_CFLAGS =		\
	$(XMZ_DEBUG_FLAGS)			\
	$(XMZ_CFLAGS)				\
	$(LIBXMZ_CFLAGS)			\
	-g					\
	-w

tests_libxmz_test_libxmz_LDADD =		\
	libxmz/libxmz-1.0.la			\
	$(LIBXMZ_LIBS)

tests/libxmz/support-%.vala: tests/support/%.vala
	cp "$<" "$@"

TESTS_XMZ_TEST_LIBXMZ_COPIED_SOURCES = 		\
	tests/libxmz/support-test.vala			\
	tests/libxmz/support-main.vala			\
	$(NULL)

tests_libxmz_test_libxmz_SOURCES =			\
	tests/libxmz/main.vala				\
	$(TESTS_XMZ_TEST_LIBXMZ_COPIED_SOURCES)	\
	$(XMZ_PLUGIN_VAPISOURCES)

CLEANFILES += 							\
	$(TESTS_XMZ_TEST_LIBXMZ_COPIED_SOURCES)		\
	$(TESTS_XMZ_TEST_LIBXMZ_COPIED_SOURCES:%.vala=%.c)

# vi:ts=8:noet
