using Gtk;
using XMZ;

class TestSensorList : Gtk.Window {
  // gettext
  private const string version = Config.VERSION;


  public TestSensorList () {
    // Prepare Gtk.Window
    this.title = "Test Sensor List";
    this.destroy.connect (Gtk.main_quit);
    this.set_default_size (1024, 600);

    // Model
    GenericArray<Sensor> sensors;
    SensorController controller = new SensorController ();
    sensors = controller.get_sensors ();
    SensorModel model = new SensorModel (sensors);

    // View
    Gtk.TreeView view = new Gtk.TreeView.with_model (model);
    view.insert_column_with_attributes (-1, _("Name"), new Gtk.CellRendererText (), "text", 0);
    view.insert_column_with_attributes (-1, _("ADC_Value"), new Gtk.CellRendererText (), "text", 1);
    this.add (view);
  }


  public static int main (string[] args) {
    Gtk.init (ref args);

    TestSensorList app = new TestSensorList ();
    app.show_all ();
    Gtk.main ();

    return 0;
  }
}
