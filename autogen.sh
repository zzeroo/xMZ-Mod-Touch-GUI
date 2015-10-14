#!/bin/sh

set -e

test -n "$srcdir" || srcdir=`dirname "$0"`
test -n "$srcdir" || srcdir=.

olddir=`pwd`
cd "$srcdir"

[ ! -f README ] && ln README.md README
[ ! -d benchmark ] && mkdir benchmark

PKG_NAME=`autoconf --trace 'AC_INIT:$1' "$srcdir/configure.ac"`

if [ "$#" = 0 -a "x$NOCONFIGURE" = "x" ]; then
	echo "**Warning**: I am going to run \`configure' with no arguments." >&2
	echo "If you wish to pass any to it, please specify them on the" >&2
	echo \`$0\'" command line." >&2
	echo "" >&2
fi

set -x
#aclocal --install || exit 1
autoreconf --verbose --force --install -Wno-portability || exit 1
set +x

cd "$olddir"

if test -z "$NOCONFIGURE"; then
  "$srcdir"/configure "$@"
fi
