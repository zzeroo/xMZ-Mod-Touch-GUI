include $(top_srcdir)/Makefile.decl

noinst_PROGRAMS = $(TEST_PROGS)

TEST_PROGS += tests

xmz_mod_touch_gui_SOURCES = \
														../src/SensorNode.vala \
														../src/SensorModel.vala \
														$(NULL)

tests_VALASOURCES = \
										$(xmz_mod_touch_gui_SOURCES) \
										test-example.vala \
										test-sensor-model.vala \
										testmain.vala \
										testcase.vala \
										--target-glib=$(GLIB_REQUIRED) \
										$(NULL)

tests_SOURCES = \
								$(tests_VALASOURCES:.vala=.c) \
								$(NULL)

tests_DEPENDENCIES = \
										 tests_vala.stamp \
										 $(NULL)

tests_CFLAGS = \
							 -w \
							 -rdynamic \
							 $(NULL)

tests_CPPFLAGS = \
								 -I$(top_srcdir)/src \
								 $(GLIB_CFLAGS) \
								 $(XMZ_MOD_TOUCH_GUI_CFLAGS) \
								 $(NULL)

tests_LDADD = \
							$(GTK_LIBS) \
							$(GLIB_LIBS) \
							$(NULL)

tests_VALAFLAGS = \
									--basedir $(top_srcdir) \
									--vapidir $(top_srcdir)/vapi \
									--pkg posix \
									--pkg gio-2.0 \
									--pkg gtk+-3.0 \
									--pkg gee-0.8 \
									--pkg gmodule-2.0 \
									--pkg sqlite3 \
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
