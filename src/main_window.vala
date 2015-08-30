class Gui : Gtk.Application {
  public Gui () {
    Object (application_id: "com.gaswarnanlagen.xmz-mod-touch", flags: ApplicationFlags.FLAGS_NONE);
  }

  protected override void activate () {
    //Gtk.Settings.get_default ().gtk_application_prefer_dark_theme = true;
    Gtk.ApplicationWindow window = new Gtk.ApplicationWindow (this);
    window.set_default_size (1024, 600);
    window.set_title ("xMZ-Mod-Touch");

    /* Primary Box */
    Gtk.Box primary_box = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 0);

    /* Inner Box */
    Gtk.Box inner_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 0);
    primary_box.pack_end (inner_box, true);

    /* Stack */
    Gtk.Stack stack = new Gtk.Stack ();
    stack.set_transition_type (Gtk.StackTransitionType.SLIDE_LEFT_RIGHT); // SLIDE_UP

    for (int i = 0; i < 10; i++) {
      var sensor_builder = new Gtk.Builder ();
      sensor_builder.add_from_file ("./sensor.glade");
      var grid_sensor = sensor_builder.get_object ("grid_sensor") as Gtk.Grid;
      stack.add_titled (grid_sensor, "sensor" + i.to_string (), "Sensor " + i.to_string ());
    }

    inner_box.pack_end (stack, true);

    /* StackSidebar */
    Gtk.StackSidebar stack_sidebar = new Gtk.StackSidebar ();
    stack_sidebar.set_stack (stack);

    primary_box.pack_end (stack_sidebar, true);


    Gtk.GestureSwipe gesture = new Gtk.GestureSwipe (stack);
    gesture.set_touch_only (true);
    gesture.swipe.connect ((p0, p1) => {
        message (p0.to_string ());
        message (p1.to_string ());
    });


    window.add (primary_box);
    window.show_all ();
  }
}

int main (string[] args) {
   return new Gui ().run (args);
}
