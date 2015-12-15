namespace XMZ {

public class SensorController {

  public SensorController () {
  }

  ~SensorController () {
  }


  public Sensor get_sensor (uint id) {
    var sensor = new Sensor ("CO/NO² Sensor");

    return sensor;
  }

  public GenericArray<Sensor> get_sensors () {
    GenericArray<Sensor> sensors = new GenericArray<Sensor> ();

    for (int i = 1; i < 7; i++) {
      sensors.add ( new Sensor ("Sensor "+ i.to_string () + " CO") );
      sensors.add ( new Sensor ("Sensor "+ i.to_string () + " NO²") );
    }

    return sensors;
  }
}
}
