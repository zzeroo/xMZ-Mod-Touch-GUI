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

    for (int i = 1; i < 11; i++) {
      var grid = new Gtk.Grid ();
      var lbl_name = new Gtk.Label ( "Sensor" + i.to_string ());
      grid.attach(lbl_name, 0, 0, 1, 1);
      stack.add_titled (grid, "sensor" + i.to_string (), "Sensor " + i.to_string ());
    }

    this.pack_start (stack, true);

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
    window.set_default_size (1024, 600);
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
