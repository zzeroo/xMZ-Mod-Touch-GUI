using Gtk;

public class Sensor : Object {
  // Data
  public int id {get; set;}
  public string name {get; set;}

  public Sensor (int id, string name) {
    this.name = name;
    this.id = id;
  }
}
