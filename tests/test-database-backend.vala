using GLib;
using Sqlite;
using Gee;

class DatabaseBackendTest : Gee.TestCase {

  public DatabaseBackendTest () {
    base ("DatabaseBackendTest");

    add_test ("[DatabaseBackend] example", test_example);
    add_test ("[DatabaseBackend] init_sqlite", test_init_sqlite);
  }

  private DatabaseBackend backend;

  public override void set_up () {
    backend = new DatabaseBackend ();
  }

  public override void tear_down () {
    // tear down your test
  }


  public void test_example () {
    // add your expressions
    assert (true);
  }

  public void test_init_sqlite () {
    backend.init_sqlite ();

    assert (true);
  }
}

