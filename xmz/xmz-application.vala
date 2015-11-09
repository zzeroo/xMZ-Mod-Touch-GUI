namespace XMZ {

public class Application : Gtk.Application {

  private Settings d_state_settings;

  public Settings state_settings {
    owned get { return d_state_settings; }
  }

  public Application () {
    Object (application_id: "com.gaswarnanlagen.xmz",
            flags:  ApplicationFlags.SEND_ENVIRONMENT);
  }

  private struct Options {
    public static bool quit = false;
    public static string activity;
  }

  static construct {
    Options.activity = "";
  }

  private static bool show_version_and_quit () {
    stdout.printf ("%s %s\n",
                   Environment.get_application_name (),
                   Config.VERSION);

    Options.quit = true;
    return true;
  }


  private void on_app_new_window_activated () {
    new_window ();
  }

  private void on_app_help_activated () {}

  private void on_app_about_activated () {
    string[] artists = {"Stefan Müller <s.mueller@it.kls-glt.de>"};
    string[] authors = {"Stefan Müller <s.mueller@it.kls-glt.de>"};
    string copyright = "Copyright \xc2\xa9 2015 RA-Gas GmbH";
    string comments = _("xmz Graphical User Interface for xMesszentrale mit Modbus Interface und Touchscreen");

    unowned List<Gtk.Window> wnds = get_windows ();

    Gtk.show_about_dialog (wnds != null ? wnds.data : null,
                           "artists", artists,
                           "authors", authors,
                           "copyright", copyright,
                           "comments", comments,
                           "translator-credits", _("translator-credits"),
                           "version", Config.VERSION,
                           "website", Config.PACKAGE_URL,
                           "website-label", _("xmz homepage"),
                           "logo-icon-name", Config.PACKAGE_NAME,
                           "license-type", Gtk.License.GPL_3_0);
  }

  private void on_app_quit_activated () {
    var wnds = get_windows ().copy ();

    foreach (var window in wnds) {
      window.destroy ();
    }
  }


  private void init_error (string msg) {
    var dlg = new Gtk.MessageDialog (null,
                                     0,
                                     Gtk.MessageType.ERROR,
                                     Gtk.ButtonsType.CLOSE,
                                     "%s",
                                     msg);

    dlg.window_position = Gtk.WindowPosition.CENTER;

    dlg.response.connect(() => { Gtk.main_quit(); });
    dlg.show ();
  }

  protected override void startup () {
    base.startup ();

    try {
      XMZ.init ();
    } catch (Error e) {
      error ("%s", e.message);
    }

    // Use our own css provider
    Gtk.CssProvider? provider = Resource.load_css ("style.css");

    if (provider != null) {
      Gtk.StyleContext.add_provider_for_screen (Gdk.Screen.get_default (),
                                                provider,
                                                600);

      var theme = Gtk.IconTheme.get_default ();
      // theme.prepend_search_path (Path.build_filename (Config.XMZ_DATADIR, "icons"));
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
     var window = Window.create_new (this);

     present_window ();
  }

  private void present_window (string? activity = null) {
    /* Present the last window in the windows registered on the
       application. If there is no windows, then create a new empty
       window.
    */
    unowned List<Gtk.Window> windows = get_windows ();

    if (windows == null) {
      new_window ();
      return;
    }

    var w = (XMZ.Window)windows.first ().data;
    w.present (activity);
  }
}
}
