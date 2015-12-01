using Gtk;
using XMZ;

class TestSensorList {

  public static int main (string[] args) {

    Gtk.init (ref args);

    var window = new Window ();
    window.set_default_size (1024, 600);
    window.add (new SensorListView (new SensorModel ()));
    window.show_all ();

    window.delete_event.connect ((w, ev) => {
                                 Gtk.main_quit ();
                                 return true;
                                 });

    Gtk.main ();

    return 0;
  }
}
