
namespace XMZ {

public class Sensor : Object {

  public string name { get; construct; }

  public string description { get; set; }

  public int adc_value { get; set; }

  public int adc_at_nullgas { get; set; }

  public int adc_at_messgas { get; set; }


  public Sensor (string name) {
    Object (name: name);
  }


  construct {}

}
}

// ex:set ts=4 noet
