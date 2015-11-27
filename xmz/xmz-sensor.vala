namespace XMZ {

public class Sensor : Object {
  // getext hack
  private const string version = Config.VERSION;

  public string name { get; set; }
  public int adc_value { get; set; }

  public Sensor (string name = _("unknown sensor"), int adc_value = -1) {
    this.name = name;
    this.adc_value = adc_value;
  }
}
}
