 autoheader \
    && aclocal \
    && libtoolize --ltdl --copy --force \
    && automake --add-missing --copy \
    && autoconf \
    && ./configure

