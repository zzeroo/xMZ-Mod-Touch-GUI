namespace XMZ {

public enum SensorModelColumns {
  NAME,
  ADC_VALUE,
  NUM;

  public Type type () {
    switch (this) {
      case NAME:
      // case DESCRIPTION:
        return typeof (string);
      case ADC_VALUE:
        return typeof (int);
      default:
        break;
    }
    return Type.INVALID;
  }
}

public class SensorModel : Object {

  public string name { get; construct; }
  public int adc_value { get; set;}

  public SensorModel (string name) {
    Object (name: name);
  }

  construct {
  }
}
}
