namespace XMZ {

public class SensorController {

  public static SensorModel get_sensor () {
    var sensor = new SensorModel ("Sensor1 CO");

    return sensor;
  }

  public static SensorModel[] get_sensors () {
    SensorModel[] sensors = {};

    for (int i = 0; i < 1000; i++) {
      sensors += new SensorModel ("Sensor"+ i.to_string () + " CO");
      sensors += new SensorModel ("Sensor"+ i.to_string () + " NO²");
    }

    return sensors;
  }
}
}
