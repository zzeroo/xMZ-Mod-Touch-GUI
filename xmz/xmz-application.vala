namespace XMZ {

public class Application : Gtk.Application {

  public Application () {
    Object (application_id: "com.gaswarnanlagen.xmz",
            flags:  ApplicationFlags.SEND_ENVIRONMENT);
  }

  static construct {
    assert (true);
  }

  //private void on_app_new_window_activated () {
    //new_window ();
  //}

  protected override void startup () {
    base.startup ();

    // FIXME: Is here the right place for network/ database update thread startup?
    //try {
      //XMZ.init ();
    //} catch (Error e) {
      //error ("%s", e.message);
    //}

    // Use our own css provider
    Gtk.CssProvider? provider = Resource.load_css ("style.css");
    if (provider != null) {
      Gtk.StyleContext.add_provider_for_screen (Gdk.Screen.get_default (),
                                                provider,
                                                600);
      var theme = Gtk.IconTheme.get_default ();
      theme.prepend_search_path (Path.build_filename (Config.XMZ_DATADIR, "icons"));
    }
  }

  protected override void shutdown () {
    base.shutdown ();
  }

  protected override void activate () {
    present_window ();
    base.activate ();
  }

  private void new_window () {
    Window.create_new (this);

    present_window ();
  }

  private void present_window () {
    /* Present the last window in the windows registered on the
       application. If there is no window, then create a new empty
       window.
       */
    unowned List<Gtk.Window> windows = get_windows ();
    if (windows == null) {
      new_window ();
      return;
    }

    var w = (XMZ.Window) windows.first ().data;
    w.present ();
  }

}
}
