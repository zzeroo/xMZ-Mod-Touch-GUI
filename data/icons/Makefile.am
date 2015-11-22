data_icons_public_icons_themes = \
	hicolor

data_icons_public_icons = \
	hicolor_apps_16x16_xmz.png \
	hicolor_apps_22x22_xmz.png \
	hicolor_apps_24x24_xmz.png \
	hicolor_apps_32x32_xmz.png \
	hicolor_apps_48x48_xmz.png \
	hicolor_apps_256x256_xmz.png \
	hicolor_apps_scalable_xmz-symbolic.svg

data_icons_gtk_update_icon_cache = $(GTK_UPDATE_ICON_CACHE) -f -t

data-icons-update-icon-cache:
	@-if test -z "$(DESTDIR)"; then \
		echo "Updating Gtk icon cache."; \
		for theme in $(data_icons_public_icons_themes); do \
			$(MKDIR_P) $(datadir)/xmz/icons/$$theme; \
			$(data_icons_gtk_update_icon_cache) $(datadir)/icons/$$theme; \
		done; \
	else \
		echo "*** Icon cache not updated.  After (un)install, run this:"; \
		for theme in $(data_icons_public_icons_themes); do \
			echo "***   $(data_icons_gtk_update_icon_cache) $(datadir)/icons/$$theme"; \
		done; \
	fi

data-icons-install-icons:
	@echo "Installing icons"; \
	for icon in $(data_icons_public_icons); do \
		THEME=`echo $$icon | cut -d_ -f1`; \
		CONTEXT=`echo $$icon | cut -d_ -f2`; \
		SIZE=`echo $$icon | cut -d_ -f3`; \
		ICONFILE=`echo $$icon | cut -d_ -f4`; \
		$(MKDIR_P) $(DESTDIR)$(datadir)/icons/$$THEME/$$SIZE/$$CONTEXT; \
		echo "  - $(DESTDIR)$(datadir)/icons/$$THEME/$$SIZE/$$CONTEXT/$$ICONFILE"; \
		$(INSTALL_DATA) $(srcdir)/data/icons/$$icon $(DESTDIR)$(datadir)/icons/$$THEME/$$SIZE/$$CONTEXT/$$ICONFILE; \
	done;

data-icons-uninstall-icons:
	-for icon in $(data_icons_public_icons); do \
		THEME=`echo $$icon | cut -d_ -f1`; \
		CONTEXT=`echo $$icon | cut -d_ -f2`; \
		SIZE=`echo $$icon | cut -d_ -f3`; \
		ICONFILE=`echo $$icon | cut -d_ -f4`; \
		rm -f $(DESTDIR)$(datadir)/icons/$$THEME/$$SIZE/$$CONTEXT/$$ICONFILE; \
	done;

install-data-local: data-icons-install-icons
install-data-hook: data-icons-update-icon-cache

uninstall-local: data-icons-uninstall-icons
uninstall-hook: data-icons-update-icon-cache

EXTRA_DIST += $(addprefix data/icons/,$(data_icons_public_icons))

# vi:ts=8:noet

