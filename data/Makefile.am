data_desktopdir = $(datadir)/applications
data_desktop_in_files = data/xmz.desktop.in
data_desktop_DATA = $(data_desktop_in_files:.desktop.in=.desktop)

data_appdatadir = $(datadir)/appdata
data_appdata_DATA = data/xmz.appdata.xml

dist_man_MANS = data/xmz.1

gsettings_SCHEMAS += data/com.gaswarnanlagen.xmz.gschema.xml

if GLADE_CATALOG
data_catalogdir = $(GLADE_CATALOG_DIR)
dist_data_catalog_DATA = data/xmz-glade.xml
endif

EXTRA_DIST +=					\
	$(data_desktop_in_files)		\
	data/xmz.appdata.xml			\
	data/com.gaswarnanlagen.xmz.gschema.xml.in.in

CLEANFILES += 					\
	$(data_desktop_DATA)			\
	data/com.gaswarnanlagen.xmz.gschema.xml

DISTCLEANFILES +=				\
	$(data_desktop_DATA)			\
	data/com.gaswarnanlagen.xmz.gschema.xml

include data/icons/Makefile.am

# vi:ts=8:noet
