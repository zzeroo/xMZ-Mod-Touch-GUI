namespace XMZ {

public class SensorController {

  public static Sensor get_sensor () {
    var sensor = new Sensor ("Sensor1 CO");

    return sensor;
  }

  public static Sensor[] get_sensors () {
    Sensor[] sensors = {};

    for (int i = 0; i < 1000; i++) {
      sensors += new Sensor ("Sensor "+ i.to_string () + " CO");
      sensors += new Sensor ("Sensor "+ i.to_string () + " NO²");
    }

    return sensors;
  }
}
}
