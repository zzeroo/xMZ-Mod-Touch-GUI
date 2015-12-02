namespace XMZ {

public class Sensor : Object {

  public string name { get; set; }
  public int adc_value { get; set; }

  public Sensor (string name = "unknown sensor", int adc_value = -1) {
    this.name = name;
    this.adc_value = adc_value;
  }
}
}
