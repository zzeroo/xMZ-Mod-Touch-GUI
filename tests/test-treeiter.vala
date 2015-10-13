public class MySensors : Object {
    // Create a ListStore
  public Gtk.ListStore list_store = new Gtk.ListStore (2, typeof (int), typeof (string) );
  public Gtk.TreeIter iter;

  public MySensors () {
    GenericArray<SensorNode> data = new GenericArray<SensorNode> ();
    data.add (new SensorNode (1, "Sensor 1 (CO)"));
    data.add (new SensorNode (2, "Sensor 2 (CO)"));
    data.add (new SensorNode (3, "Sensor 3 (CO/NO2)"));
    data.add (new SensorNode (4, "Sensor 4 (CO/NO2)"));
    data.add (new SensorNode (5, "Sensor 5 (CO/NO2)"));

    data.foreach ((sensor) => {
                  // Insert data (0: id, 1: Name)
                  list_store.append (out iter);
                  list_store.set (iter, 0, sensor.id, 1, sensor.name);
                  });
  }

  public void run () {
    for (bool next = list_store.get_iter_first (out iter); next; next = list_store.iter_next (ref iter)) {
      Value id, name;
      list_store.get_value (iter, 0, out id);
      list_store.get_value (iter, 1, out name);
      stdout.printf ("%d\t%s\n", (int)id, (string)name);
    }
  }

  public static int main (string[] args) {
    var app = new MySensors ();
    app.run ();

    return 0;
  }
}
