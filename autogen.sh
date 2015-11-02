#!/bin/sh
# Run this to generate all the initial makefiles, etc.

test -n "$srcdir" || srcdir=`dirname "$0"`
test -n "$srcdir" || srcdir=.

olddir=`pwd`
cd "$srcdir"

#[ ! -f README ] && ln README.md README
[ ! -f ChangeLog ] && touch ChangeLog
[ ! -f NEWS ] && touch NEWS
[ ! -f README ] && touch README
[ ! -f AUTHORS ] && touch AUTHORS

INTLTOOLIZE=`which intltoolize`
if test -z $INTLTOOLIZE; then
        echo "*** No intltoolize found, please install the intltool package ***"
        exit 1
fi

AUTORECONF=`which autoreconf`
if test -z $AUTORECONF; then
        echo "*** No autoreconf found, please install it ***"
        exit 1
fi

if test -z `which autopoint`; then
        echo "*** No autopoint found, please install it ***"
        exit 1
fi


autoreconf --verbose --force --install -Wno-portability || exit 1


cd "$olddir"
test -n "$NOCONFIGURE" || "$srcdir/configure" "$@"
