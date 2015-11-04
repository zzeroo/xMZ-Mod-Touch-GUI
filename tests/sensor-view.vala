using Gtk;
using XMZ;

class TestSensorView {
  public static int main (string[] args) {
    Gtk.init (ref args);

    try {
      XMZ.init ();
    } catch (Error e) {
      stderr.printf ("Failed to initalize xmz: %s\n", e.message);
      return 1;
    }

    var Window = new Window ();
    window.set_default_size (1024, 600);

    window.show_all ();

    window.delete_event_connect ((w, ev) => {
                                 Gtk.main_quit ();
                                 return true;
                                 });

    Gtk.main ();

    return 0;
  }
}
