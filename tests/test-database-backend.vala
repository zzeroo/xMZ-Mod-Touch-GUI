using GLib;
using Sqlite;
using Gee;

class DatabaseBackendTest : Gee.TestCase {

  public DatabaseBackendTest () {
    base ("DatabaseBackendTest");

    add_test ("[DatabaseBackend] init_sqlite", test_init_sqlite);
    add_test ("[DatabaseBackend] store_sensor (SensorNode sensor)", test_store_sensor);
    //add_test ("[DatabaseBackend] get_sensor (id)", test_get_sensor);
    //add_test ("[DatabaseBackend] get_sensors", test_get_sensors);
  }

  private DatabaseBackend backend;

  private void drop_database_file () {
    try {
      var database = File.new_for_path ("database.db");
      database.delete ();
    } catch (Error e) {
      stderr.printf ("Error: %s\n", e.message);
    }
  }

  public override void set_up () {
    drop_database_file ();
    backend = new DatabaseBackend ();
  }

  public override void tear_down () {
  }



  public void test_init_sqlite () {
    assert (backend.init_sqlite () == 0);
  }

  public void test_store_sensor () {
    backend.init_sqlite ();
    var sensor = new SensorNode (1337, "Testsensor");

    // Return value after save is 0
    assert (backend.store_sensor (sensor) == 0);
    // Before we start the test sensor is not in the database
    // FIXME: fix this assert (in database) for check in database
    // after backend.store_sensor the stored sensor id must be the same
    // as of the test sensor
    assert (backend.get_sensor (0).id == sensor.id );
    assert (backend.get_sensor (0).name == sensor.name );
    assert (backend.get_sensor (0).adc_value == sensor.adc_value );
    assert (backend.get_sensor (0).adc_messgas == sensor.adc_messgas );
    assert (backend.get_sensor (0).adc_nullgas == sensor.adc_nullgas );
  }

  public void test_get_sensor () {
    backend.init_sqlite ();
    var sensor = new SensorNode (1, "Sensor 1");

    assert (backend.get_sensor (1).id == sensor.id );
  }

  public void test_get_sensors () {
    backend.init_sqlite ();
    GenericArray<SensorNode> sensors = new GenericArray<SensorNode> ();

    sensors = backend.get_sensors ();

    assert (sensors[0].id == 1);
    assert (sensors[0].name == "Sensor 1");
    assert (sensors[1].id == 2);
    assert (sensors[1].name == "Sensor 2");
  }
}

