namespace XMZ {

public class Application : Gtk.Application {

  public Application () {
    Object (application_id: "com.gaswarnanlagen.xmz",
            flags:  ApplicationFlags.SEND_ENVIRONMENT);
  }

  private void on_app_new_window_activated () {
    new_window ();
  }

  private void on_app_quit_activated () {
    var wnds = get_windows ().copy ();

    foreach (var window in wnds) {
      window.destroy ();
    }
  }

  protected override void startup () {
    base.startup ();

    try {
      XMZ.init ();
    } catch (Error e) {
      error ("%s", e.message);
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
