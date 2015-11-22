namespace XMZ {

public errordomain InitError {
  THREADS_UNSAVE
}

private static bool xmz_inited = false;
private static InitError? xmz_initerr = null;

public void init () throws Error {
  if (xmz_inited) {
    if (xmz_initerr != null) {
      throw xmz_initerr;
    }

    return;
  }

  xmz_inited = true;

  XMZ.init ();

  // Add our own css provider
  Gtk.CssProvider? provider = XMZ.Resource.load_css ("libxmz-style.css");

  if (provider != null) {
    Gtk.StyleContext.add_provider_for_screen (Gdk.Screen.get_default (),
                                              provider,
                                              600);
  }
}
}

// ex:set ts=4 noet
