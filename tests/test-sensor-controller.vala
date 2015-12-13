/**
  */
namespace XMZ {

class TestSensorController : Gee.TestCase {

  private SensorModel sensor;

  public TestSensorController () {
    base ("TestSensorController");
    add_test ("get_sensors", get_sensors);
    add_test ("get_sensor", get_sensor);
    add_test ("create_sensor", create_sensor);
  }

  public override void set_up () {
    //sensor = new XMZ.SensorModel ("CO/NO", 1);
  }

  public override void tear_down () {
  }

  public void get_sensors () {
    assert (true);
  }

  public void get_sensor () {
    assert (true);
  }

  public void create_sensor () {
    assert (true);
  }
}
}
