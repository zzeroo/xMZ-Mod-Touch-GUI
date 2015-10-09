// SensorNode object
// http://references.valadoc.org/#!api=gtk+-3.0/Gtk.TreeModel
public class SensorNode : Object {
  // Data
  public int id { get; set; }
  public string name { get; set; }

  public SensorNode (int id, string name) {
    this.id = id;
    this.name = name;
  }
}
