
namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-window.ui")]
public class Window : Gtk.ApplicationWindow, XMZExt.Application, Initable {

  private Settings d_state_settings;
  private Settings d_interface_settings;

  private UIElements<XMZExt.Activity> d_activities;
  private Notifications d_notifications;

  // Widgets
  [GtkChild]
  private Gtk.Stack d_main_stack;
  [GtkChild]
  private Gtk.Stack d_stack_activities;
  [GtkChild]
  private SettingsView d_settings_view;
  [GtkChild]
  private SensorView d_sensor_view;
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
	d_interface_settings = new Settings ("com.gaswarnanlagen.xmz.preferences.interface");
	d_notifications = new Notifications (d_overlay);

	d_settings_view.application = this;

	d_infobar.response.connect ((w, r) => {
								d_infobar.hide();
								});
  }

  private void on_close_activated () {
	close ();
  }

  public XMZExt.Activity? current_activity {
	owned get {
	  if (d_mode == Mode.ACTIVITY) {
		return d_sensor_view;
	  } else {
		return d_settings_view;
	  }
	}
  }

  /**
   * This action is fired when the big button top right was hit
   */
  [GtkCallback]
  private void settings_button_clicked () {
	if (d_mode == Mode.SETTINGS) {
	  d_mode = Mode.ACTIVITY;

	  d_main_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
	  d_main_stack.set_visible_child (d_sensor_view);
	} else {
	  d_mode = Mode.SETTINGS;

	  d_main_stack.transition_type = Gtk.StackTransitionType.SLIDE_RIGHT;
	  d_main_stack.set_visible_child (d_settings_view);
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

	d_mode = Mode.ACTIVITY;

	d_main_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
	d_main_stack.set_visible_child (d_sensor_view);

	return true;
  }

  public static Window? create_new (Gtk.Application app) {

	Window? ret = new Window ();

	if (ret != null) {
	  ret.application = app;
	}

	try {
	  ((Initable) ret).init (null);
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

  public void show_infobar (string title,
							string message,
							Gtk.MessageType type) {

	Idle.add (() => {
			  var primary = "<b>%s</b>".printf (Markup.escape_text (title));
			  var secondary = "<small>%s</small>".printf (Markup.escape_text (message));

			  d_infobar_primary_label.set_label (primary);
			  d_infobar_secondary_label.set_label (secondary);
			  d_infobar.message_type = type;

			  d_infobar.show ();
			  return false;
			  });
  }

  public XMZExt.Notifications notifications {
	owned get { return d_notifications; }
  }


}
}

// ex:ts=4 noet
