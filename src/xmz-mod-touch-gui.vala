using Gtk;

class SettingsWindow : Gtk.Box {

  public SettingsWindow () {
    this.pack_start (new Gtk.Label ("1"), false, false, 0);
    this.pack_start (new Gtk.Label ("2"), false, false, 0);
  }
}

class MainWindow : Gtk.Box {

  public MainWindow (){
    /* Stack */
    var stack = new Gtk.Stack ();
    stack.set_transition_type (Gtk.StackTransitionType.SLIDE_LEFT_RIGHT);

    // FIXME: Create Sensor Object
    for (int i = 1; i < 48; i++) {
      // Some random values
      var sensor_value = Random.int_range (1, 300);
      var grid = new Gtk.Grid ();
      var lbl_name = new Gtk.Label ( "Sensor" + i.to_string ());
      lbl_name.get_style_context ().add_class ("sensor_name");
      lbl_name.set_hexpand (true);
      lbl_name.set_vexpand (true);
      lbl_name.set_halign (Gtk.Align.START);
      var levelbar_value = new Gtk.LevelBar.for_interval ( 0, 300 );
      levelbar_value.set_vexpand (true);
      levelbar_value.set_orientation (Gtk.Orientation.VERTICAL);
      levelbar_value.set_inverted (true);
      levelbar_value.set_value (sensor_value);
      levelbar_value.get_style_context ().add_class ("sensor_value");
      var lbl_value = new Gtk.Label (sensor_value.to_string ());
      lbl_value.set_halign (Gtk.Align.START);
      var lbl_value_si = new Gtk.Label ("ppm");
      lbl_value_si.set_halign (Gtk.Align.START);
      var lbl_average_title = new Gtk.Label ("Mittelwerte");
      var lbl_average15_title = new Gtk.Label ("15 Minuten:");
      var lbl_average15_value = new Gtk.Label ("000");
      var lbl_average15_si =    new Gtk.Label ("ppm");
      var lbl_average30_title = new Gtk.Label ("30 Minuten:");
      var lbl_average30_value = new Gtk.Label ("000");
      var lbl_average30_si =    new Gtk.Label ("ppm");
      var lbl_average60_title = new Gtk.Label ("60 Minuten:");
      var lbl_average60_value = new Gtk.Label ("000");
      var lbl_average60_si =    new Gtk.Label ("ppm");

      grid.attach (lbl_name,            0, 0, 10, 1);
      grid.attach (levelbar_value,           0, 1, 1, 5);
      grid.attach (lbl_value,           2, 4, 4, 2);
      grid.attach (lbl_value_si,        6, 5, 2, 1);
      grid.attach (lbl_average_title,   6, 1, 4, 1);
      grid.attach (lbl_average15_title, 6, 2, 2, 1);
      grid.attach (lbl_average15_value, 8, 2, 1, 1);
      grid.attach (lbl_average15_si,    9, 2, 1, 1);
      grid.attach (lbl_average30_title, 6, 3, 2, 1);
      grid.attach (lbl_average30_value, 8, 3, 1, 1);
      grid.attach (lbl_average30_si,    9, 3, 1, 1);
      grid.attach (lbl_average60_title, 6, 4, 2, 1);
      grid.attach (lbl_average60_value, 8, 4, 1, 1);
      grid.attach (lbl_average60_si,    9, 4, 1, 1);

      stack.add_titled (grid, "sensor" + i.to_string (), "Sensor " + i.to_string ());
    }

    this.pack_start (stack, true, true);

    /* StackSidebar */
    var stack_sidebar = new Gtk.StackSidebar ();
    stack_sidebar.set_stack (stack);

    this.pack_end (stack_sidebar, false);
  }

}


class XmzModTouchGui : Gtk.Application {

  public XmzModTouchGui () {
    Object (application_id: "com.gaswarnanlagen.xmz-mod-touch-gui", flags: ApplicationFlags.FLAGS_NONE);
  }

  protected override void activate () {
    //Gtk.Settings.get_default ().gtk_application_prefer_dark_theme = true;
    var window = new Gtk.ApplicationWindow (this);
    if (GLib.Environment.get_variable("XMZ_HARDWARE") == "0.1.0") {
      window.maximize ();
    } else {
      window.set_default_size (1024, 600);
    }
    // CSS Provider
    var css_provider = new CssProvider ();
    css_provider.load_from_resource ("/com/gaswarnanlagen/xmz-mod-touch-gui/xmz-mod-touch-gui.css");
    StyleContext.add_provider_for_screen (Gdk.Screen.get_default (), css_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Header bar
    var header_bar = new Gtk.HeaderBar ();
    header_bar.set_title ("xMZ-Mod-Touch");
    header_bar.set_subtitle ("xMesszentrale mit Modbus Interface und Touchscreen");
    header_bar.set_show_close_button (false);
    var settings_button = new Gtk.Button.from_icon_name ("emblem-system", Gtk.IconSize.LARGE_TOOLBAR);
    header_bar.pack_start (settings_button);

    var window_stack = new Gtk.Stack ();
    window_stack.set_transition_type (Gtk.StackTransitionType.SLIDE_DOWN); // SLIDE_LEFT_RIGHT, SLIDE_UP

    // var button1 = new Gtk.Button.with_label ("Button1");
    // window_stack.add_named (button1, "Main Window");
    var main_window = new MainWindow () as Gtk.Box;
    window_stack.add_named (main_window, "Main Window");

    var settings_window = new SettingsWindow () as Gtk.Box;
    window_stack.add_named (settings_window, "Settings Window");

    // Toggle "Main Window" and "Settings Window"
    settings_button.clicked.connect (() => {
        switch (window_stack.get_visible_child_name ()) {
          case "Main Window":
            window_stack.set_visible_child_name ("Settings Window");
            break;
          case "Settings Window":
            window_stack.set_visible_child_name ("Main Window");
            break;
          default:
            window_stack.set_visible_child_name ("Main Window");
            break;
        }
    }); // End Toggle Main/ Settings Window


    window.add (window_stack);
    window.set_titlebar (header_bar);
    window.show_all ();
  }
}

int main (string[] args) {
  return new XmzModTouchGui ().run (args);
}
