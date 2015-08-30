using Gtk;

int main (string[] args) {

  Gtk.init (ref args);
  var time = new TimeoutSource(100);

  //Gtk.Settings.get_default().set("gtk-application-prefer-dark-theme", true);
  Gtk.Settings.get_default().set("gtk-font-name", "DejaVu Sans 18");

  try {
    // if the UI contains custom widgets their type
    // must have been instanticated once.
    //
    // Type Type = typeof(Foo.BarEntry);
    // assert (type != 0);
    var builder = new Builder ();
    builder.add_from_file ("./basic_window.glade");
    var window = builder.get_object ("basic_window") as Window;
    window.maximize ();
    window.set_default_size (1024, 600);


    Stack stack = builder.get_object ("stack") as Stack;

    var sensor_builder = new Builder ();
    sensor_builder.add_from_file ("./sensor.glade");

    var grid_sensor1 = sensor_builder.get_object ("grid_sensor") as Grid;
    stack.add_titled (grid_sensor1, "Sensor1", "Sensor1");

    var grid_sensor2 = sensor_builder.get_object ("grid_sensor") as Grid;
    stack.add_titled (grid_sensor2, "Sensor2", "Sensor2");

    var grid_sensor3 = sensor_builder.get_object ("grid_sensor") as Grid;
    stack.add_titled (grid_sensor3, "Sensor3", "Sensor3");


    var stack_sidebar = builder.get_object ("stacksidebar") as StackSidebar;
    stack_sidebar.set_stack (stack);

    window.show_all ();

    ///// Update the levelbar and the label
    //var i = 0;
    //time.set_callback (() => {
    //    if (i > 99) {
    //      i = 0;
    //      } else{
    //      levelbar_sensor1.set_value (i++);
    //      lbl_value_sensor1.set_text (i.to_string ("%03d"));
    //    }
    //    return true;
    //});
    //time.attach(null);

    Gtk.main ();
  } catch (Error e) {
    stderr.printf ("Could not load UI: %s\n", e.message);
    return 1;
  }

  return 0;
}
