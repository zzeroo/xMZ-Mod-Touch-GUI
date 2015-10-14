include $(top_srcdir)/Makefile.decl

noinst_PROGRAMS = $(TEST_PROGS)

TEST_PROGS += tests

tests_VALASOURCES = \
										test-example.vala \
										testmain.vala \
										testcase.vala \
										$(NULL)

tests_SOURCES = \
								$(tests_VALASOURCES:.vala=.c) \
								$(NULL)

tests_DEPENDENCIES = \
										 tests_vala.stamp \
										 $(NULL)

tests_CFLAGS = \
							 -w \
							 $(NULL)

tests_CPPFLAGS = \
								 $(GLIB_CFLAGS) \
								 $(NULL)

tests_LDADD = \
							$(GLIB_LIBS) \
							$(NULL)

tests_VALAFLAGS = \
									--basedir $(top_srcdir) \
									--vapidir $(top_srcdir)/vapi \
									--pkg gee-0.8 --pkg posix \
									$(VALAFLAGS) \
									$(NULL)

tests_vala.stamp: $(tests_VALASOURCES)
	$(AM_V_VALAC)$(VALAC) $(VALAFLAGS) $(tests_VALAFLAGS) -C $(tests_VALASOURCES)
	@touch tests_vala.stamp

$(tests_VALASOURCES:.vala=.c): tests_vala.stamp

MOSTLYCLEANFILES = \
									 $(tests_VALASOURCES:.vala=.c) \
									 tests_vala.stamp \
									 $(NULL)

EXTRA_DIST += \
							$(tests_VALASOURCES) \
							tests_vala.stamp \
							$(NULL)

coverage-reset:
	lcov --base-directory=@top_srcdir@ --directory @top_srcdir@/gee --zerocounters

coverage-report:
	lcov --directory @top_srcdir@/gee \
		--capture \
		--output-file @top_builddir@/lcov.info

	lcov --directory @top_srcdir@/gee \
		--output-file @top_builddir@/lcov.info \
		--remove @top_builddir@/lcov.info \
		"/usr/include/*" "*.c" "*.h" "*.vapi"

	$(mkdir_p) @top_builddir@/tests/coverage
	git_commit=`GIT_DIR=@top_srcdir@/.git git log -1 --pretty=format:%h 2>/dev/null`;\
	genhtml --title "@PACKAGE_STRING@ $$git_commit" \
		--output-directory @top_builddir@/tests/coverage @top_builddir@/lcov.info
	@echo
	@echo 'lcov report can be found in:'
	@echo 'file://@abs_top_builddir@/tests/coverage/index.html'
	@echo

clean-local:
	-rm -rf coverage

.PHONY: coverage-report
