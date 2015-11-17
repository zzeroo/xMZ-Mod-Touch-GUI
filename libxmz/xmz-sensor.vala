
namespace XMZ {

public class Sensor : Object {

  // Data
  public int id { get; construct; }
  public string name { get; construct; }
  public string description { get; set; }
  public int adc_value { get; set; }
  public int adc_at_nullgas { get; set; }
  public int adc_at_messgas { get; set; }


  public Sensor (int id, string name, int adc_value, int adc_at_nullgas, int adc_at_messgas) {
    Object (id:id, name:name, adc_value:adc_value, adc_at_nullgas:adc_at_nullgas, adc_at_messgas:adc_at_messgas);
  }

}
}

// ex:set ts=4 noet
