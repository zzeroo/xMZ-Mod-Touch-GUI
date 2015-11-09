
namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-window.ui")]
public class Window : Gtk.ApplicationWindow, XMZExt.Application, Initable {

  private Settings d_state_settings;
  private Settings d_interface_settings;

  private UIElements<XMZExt.Activity> d_activities;

  enum Mode {
    ACTIVITY
  }

  private Mode d_mode;


  construct {

    d_interface_settings = new Settings ("com.gaswarnanlagen.xmz.preferences.interface");
  }

  private void on_close_activated () {
    // close ();
  }

  public XMZExt.Activity? current_activity {
    owned get {
      if (d_mode == Mode.ACTIVITY) {
        return d_activities.current;
      } else {
        return d_activities.current;
      }
    }
  }


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

  public new void present (string? hint) {
    if (hint != null) {
      activate_activity (hint);
    }

    base.present ();
  }

  private bool activate_activity (string? action) {
    string default_activity;

    if (action == null || action == "") {
      default_activity = d_interface_settings.get_string ("default-activity");
    } else {
      default_activity = action;
    }

    XMZExt.Activity? def = null;

    d_activities.foreach ((element) => {
                          XMZExt.Activity activity = (XMZExt.Activity) element;

                          if (activity.is_default_for (default_activity)) {
                            def = activity;
                          }

                          return true;
                          });

    if (def != null) {
      d_activities.current = def;
      return true;
    }

    return false;
  }


}
}
