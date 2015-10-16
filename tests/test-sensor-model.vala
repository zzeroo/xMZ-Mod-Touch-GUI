using Gee;
using Gtk;

class SensorModelTest : Gee.TestCase {

  public SensorModelTest () {
    base ("SensorModelTest");

    add_test ("[SensorModel] add (id, name)", test_add);
    add_test ("[SensorModel] get_column_type (index)", test_get_column_type);
  }

  private SensorModel model;
  private TreeIter iter;

  public override void set_up () {
    model = new SensorModel ();
  }

  public override void tear_down () {
  }


  public void test_add () {
    assert (! model.get_iter_first (out iter));

    model.add (1, "Sensor 1 (CO)", 0, 0, 0);

    assert (model.get_iter_first (out iter));
  }

  public void test_get_column_type () {
    model.add (1, "Sensor 1 (CO)", 0, 0, 0);

    assert (model.get_column_type (0) == typeof (int));
    assert (model.get_column_type (1) == typeof (string));
  }

}

