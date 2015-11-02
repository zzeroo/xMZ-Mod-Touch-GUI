## Process this file with automake to produce Makefile.in

ACLOCAL_AMFLAGS = -I m4 ${ACLOCAL_FLAGS}

SUBDIRS = src data

DISTCLEANFILES = \
	intltool-extract \
	intltool-merge \
	intltool-update

EXTRA_DIST = \
	autogen.sh \
	PACKAGING

dist-hook: generate-changelog

generate-changelog:
	if test -d $(top_srcdir)/.git; then \
	  sep=`nawk 'BEGIN{$$79=OFS="-";print}'`; \
	  git --git-dir=$(top_srcdir)/.git log --date=short --stat --decorate \
	  | sed -e "s/^\(commit [abcdef0-9]\{40\}\) (.*refs\/tags\/\(v\([0-9.]\+\)\).*)/$$sep\nVersion \3\n$$sep\n\n\1/g" -e 's/^\(commit [abcdef0-9]\{40\}\).*/\1/g' | ruby -e 'puts STDIN.read.gsub(/commit\s*(.*)\nAuthor:\s*(.*)\nDate:\s*(.*)\n/, "\\3 \\2\ncommit \\1\n")' > $(distdir)/c-l; \
	  mv $(distdir)/c-l $(distdir)/ChangeLog; \
	fi

include src/Makefile.am
include data/Makefile.am
include tests/Makefile.am

EXTRA_DIST +=					\
	vapi/config.vapi			\
  $(NULL)

.PHONY: generate-changelog

-include $(top_srcdir)/git.mk
