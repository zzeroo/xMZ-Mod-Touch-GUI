namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-window.ui")]
public class Window : Gtk.ApplicationWindow, Initable {

  private bool init (Cancellable? cancellable) {
    // Settings
    var app = application as XMZ.Application;

    return true;
  }

  public static Window? create_new (Gtk.Application app) {

    Window? ret = new Window ();

    if (ret != null) {
      ret.application = app;
    }

    try {
      ((Initable)ret).init(null);
    } catch {}

    return ret;
  }

}
}
