// SensorNode object
// http://references.valadoc.org/#!api=gtk+-3.0/Gtk.TreeModel
public class SensorNode : Object {
  // Data
  public int id { get; set; }
  public string name { get; set; }
  public int adc_value { get; set; }
  public int adc_messgas { get; set; }
  public int adc_nullgas { get; set; }

  public SensorNode (int id, string name, int adc_value = 0, int adc_messgas = 0, int adc_nullgas = 0) {
    this.id = id;
    this.name = name;
    this.adc_value = adc_value;
    this.adc_messgas = adc_messgas;
    this.adc_nullgas = adc_nullgas;
  }
}
