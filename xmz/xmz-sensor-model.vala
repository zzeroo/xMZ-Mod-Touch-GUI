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
}
