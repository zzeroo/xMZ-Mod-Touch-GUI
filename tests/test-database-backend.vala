using GLib;
using Sqlite;
using Gee;

class DatabaseBackendTest : Gee.TestCase {

  public DatabaseBackendTest () {
    base ("DatabaseBackendTest");

    add_test ("[DatabaseBackend] init_sqlite", test_init_sqlite);
    add_test ("[DatabaseBackend] store_sensor (SensorNode sensor)", test_store_sensor);
    add_test ("[DatabaseBackend] get_sensor (id)", test_get_sensor);
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
    backend = new DatabaseBackend ();
  }

  public override void tear_down () {
    drop_database_file ();
  }



  public void test_init_sqlite () {
    assert (backend.init_sqlite () == 0);
  }

  public void test_store_sensor () {
    backend.init_sqlite ();
    var sensor = new SensorNode (666, "Testsensor");

    assert (backend.store_sensor (sensor) == 0);
  }

  public void test_get_sensor () {
    backend.init_sqlite ();
    var test_sensor = new SensorNode (666, "Testsensor");
    var sensor = backend.get_sensor (666);

    assert (sensor.id == test_sensor.id);
  }
}

