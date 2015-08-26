using Gtk;

int main (string[] args) {
  Gtk.init (ref args);
  //Gtk.Settings.get_default().set("gtk-application-prefer-dark-theme", true);
  Gtk.Settings.get_default().set("gtk-font-name", "DejaVu Sans 18");

  try {
    // if the UI contains custom widgets their type
    // must have been instanticated once.
    //
    // Type Type = typeof(Foo.BarEntry);
    // assert (type != 0);
    var builder = new Builder ();
    builder.add_from_file ("xMZ-Mod-Touch.glade");
    builder.connect_signals (null);
    var window = builder.get_object ("basic_window") as Window;
    window.show_all ();
    Gtk.main ();

  } catch (Error e) {
    stderr.printf ("Could not load UI: %s\n", e.message);
    return 1;
  }

  return 0;
}
