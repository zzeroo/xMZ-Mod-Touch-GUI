namespace XMZ {

public class Sensor : Object {

  public string name { get; set; }
  public int adc_value { get; set; }
  public int modbus_address { get; set; }
  public int adc_register { get; set; }

  public Sensor (string name = "unknown sensor", int adc_value = -1, int modbus_address = 1, int adc_register = 1) {
    this.name = name;
    this.adc_value = adc_value;
    this.modbus_address = modbus_address;
    this.adc_register = adc_register;
  }
}
}
