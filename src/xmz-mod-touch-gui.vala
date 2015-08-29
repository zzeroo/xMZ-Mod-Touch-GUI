using Gtk;

int main (string[] args) {
  LevelBar levelbar_sensor1;
  Label lbl_value_sensor1;

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
    builder.add_from_file ("src/xMZ-Mod-Touch.glade");
    builder.connect_signals (null);

    levelbar_sensor1 = builder.get_object ("levelbar_sensor1") as LevelBar;
    lbl_value_sensor1 = builder.get_object ("lbl_value_sensor1") as Label;
    var window = builder.get_object ("basic_window") as Window;
    window.maximize ();
    window.set_default_size (1024, 600);

    window.show_all ();

    /// Update the levelbar and the label
    var i = 0;
    time.set_callback (() => {
        if (i > 99) {
          i = 0;
          } else{
          levelbar_sensor1.set_value (i++);
          lbl_value_sensor1.set_text (i.to_string ("%03d"));
        }
        return true;
    });
    time.attach(null);

    Gtk.main ();
  } catch (Error e) {
    stderr.printf ("Could not load UI: %s\n", e.message);
    return 1;
  }

  return 0;
}
