class Gui : Gtk.Application {
  public Gui () {
    Object (application_id: "com.gaswarnanlagen.xmz-mod-touch", flags: ApplicationFlags.FLAGS_NONE);
  }

  protected override void activate () {
    //Gtk.Settings.get_default ().gtk_application_prefer_dark_theme = true;
    Gtk.ApplicationWindow window = new Gtk.ApplicationWindow (this);
    window.set_default_size (1024, 600);

    Gtk.HeaderBar header_bar = new Gtk.HeaderBar ();
    header_bar.set_title ("xMZ-Mod-Touch");
    header_bar.set_subtitle ("xMesszentrale mit Modbus Interface und Touchscreen");
    header_bar.set_show_close_button (false);
    Gtk.Button settings_button = new Gtk.Button.from_icon_name ("emblem-system", Gtk.IconSize.LARGE_TOOLBAR);
    header_bar.pack_start (settings_button);

    /* Primary Box */
    Gtk.Box primary_box = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 0);

    /* Inner Box */
    Gtk.Box inner_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 0);
    primary_box.pack_start (inner_box, true);

    /* Stack */
    Gtk.Stack stack = new Gtk.Stack ();
    stack.set_transition_type (Gtk.StackTransitionType.SLIDE_LEFT_RIGHT); // SLIDE_UP

    for (int i = 1; i < 11; i++) {
      var sensor_builder = new Gtk.Builder ();
      sensor_builder.add_from_file ("./sensor.glade");
      var grid = sensor_builder.get_object ("grid") as Gtk.Grid;
      var lbl_name = sensor_builder.get_object ("lbl_name") as Gtk.Label;
      lbl_name.set_text ("Sensor" + i.to_string ());
      stack.add_titled (grid, "sensor" + i.to_string (), "Sensor " + i.to_string ());
    }

    stack.set_border_width (40);
    inner_box.pack_start (stack, false);

    /* StackSidebar */
    Gtk.StackSidebar stack_sidebar = new Gtk.StackSidebar ();
    stack_sidebar.set_stack (stack);

    primary_box.pack_end (stack_sidebar, true);


    Gtk.GestureSwipe gesture = new Gtk.GestureSwipe (stack);
    gesture.swipe.connect ((p0, p1) => {
        message ("%l\n", p0);
        message ("%l\n", p1);
    });


    window.set_titlebar (header_bar);
    window.add (primary_box);
    window.show_all ();
  }
}

int main (string[] args) {
   return new Gui ().run (args);
}
