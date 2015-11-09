
namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-window.ui")]
public class Window : Gtk.ApplicationWindow, XMZExt.Application, Initable {

  private Settings d_state_settings;
  private Settings d_interface_settings;

  private UIElements<XMZExt.Activity> d_activities;

  // Widgets
  [GtkChild]
  private Gtk.Stack d_main_stack;
  [GtkChild]
  private Gtk.Stack d_stack_activities;
  [GtkChild]
  private Gtk.Stack d_settings_view;
  [GtkChild]
  private Gtk.Grid d_grid_main;
  [GtkChild]
  private Gtk.InfoBar d_infobar;
  [GtkChild]
  private Gtk.Label d_infobar_primary_label;
  [GtkChild]
  private Gtk.Label d_infobar_secondary_label;

  [GtkChild]
  private Gtk.Overlay d_overlay;


  enum Mode {
    SETTINGS,
    ACTIVITY
  }

  private Mode d_mode;


  construct {
    // d_interface_settings = new Settings ("com.gaswarnanlagen.xmz.preferences.interface");
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

  [GtkCallback]
  private void settings_button_clicked () {
    if (d_mode == Mode.SETTINGS) {
      d_mode = Mode.ACTIVITY;
      var button = new Gtk.Button.with_label ("Activity");

      d_main_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
      d_main_stack.set_visible_child (d_stack_activities);
      d_stack_activities.add (button);
      button.show ();
    } else {
      d_mode = Mode.SETTINGS;
      var button = new Gtk.Button.with_label ("Settings");

      d_main_stack.transition_type = Gtk.StackTransitionType.SLIDE_RIGHT;
      d_main_stack.set_visible_child (d_settings_view);
      d_settings_view.add (button);
      button.show ();
    }
  }

  private bool init (Cancellable? cancellable) {
    // Settings
    var app = application as XMZ.Application;

    if (GLib.Environment.get_variable ("XMZ_HARDWARE") == "0.1.0") {
      set_deletable (false);
      set_hide_titlebar_when_maximized (true);
      maximize ();
    } else {
      set_default_size (1024, 600);
    }

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
