/**
  */
namespace XMZ {

class TestSensorController : Gee.TestCase {

  private SensorController sensor_controller;

  public TestSensorController () {
    base ("TestSensorController");
    add_test ("sensor_instanciation", sensor_instanciation);
    add_test ("get_sensors ()", get_sensors);
    add_test ("get_sensor (uint id)", get_sensor);
    add_test ("create_sensor (Sensor)", create_sensor);
  }

  public override void set_up () {
    sensor_controller = new SensorController ();
  }

  public override void tear_down () {
  }

  public void sensor_instanciation () {
  }

  public void get_sensors () {
    var return_val = sensor_controller.get_sensors ();

    assert (return_val != null);
  }

  public void get_sensor () {
    var return_val = sensor_controller.get_sensor (1);
    assert (return_val != null);
    assert (return_val.name == "CO/NO² Sensor");
  }

  public void create_sensor () {
    assert (true);
  }
}
}
