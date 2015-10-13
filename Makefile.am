include $(top_srcdir)/Makefile.decl
ACLOCAL_AMFLAGS = --install -I m4
AM_DISTCHECK_CONFIGURE_FLAGS = --enable-introspection=no --enable-vala

#if ENABLE_DOC
#DOC_SUBDIR = \
#						 doc \
#						 $(NULL)
#endif
#
#if ENABLE_BENCHMARK
#BENCHMARK_SUBDIR = \
#									 benchmark \
#									 $(NULL)
#endif
#
SUBDIRS = \
					src \
					tests \
					$(DOC_SUBDIR) \
					$(BENCHMARK_SUBDIR) \
					$(NULL)

#if ENABLE_COVERAGE
#coverage:
#	$(MAKE) -C tests coverage-reset
#	$(MAKE) -C tests test
#	$(MAKE) -C tests coverage-report
#else
#	coverage:
#	@echo "The project has not been configured for coverage, check that you have gcov, lcov."
#	@echo "Of course, do not use --disable-coverage"
#	@exit 1
#endif
#.PHONY: coverage

EXTRA_DIST += autogen.sh




-include $(top_srcdir)/git.mk
