VALA_CFLAGS = \
							--vapidir ../vapi \
							--pkg config \
							$(VALA_PACKAGES) \
							$(NULL)

bin_PROGRAMS = xmz
xmz_SOURCES = \
							main.vala \
							$(NULL)


xmz_VALAFLAGS = \
							$(xmz_CFLAGS) \
							$(NULL)

xmz_LDADD = \
							$(xmz_LIBS) \
							$(NULL)

-include $(top_srcdir)/git.mk
