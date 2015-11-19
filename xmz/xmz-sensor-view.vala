

namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-sensor-view.ui")]
class SensorView : Gtk.Stack, XMZExt.UIElement, XMZExt.Activity {

  private const string version = Config.VERSION;
  public XMZExt.Application? application { owned get; construct set; }

  private XMZ.DatabaseBackend backend;

  private GenericArray<Sensor> sensors = new GenericArray<Sensor> ();
  private SensorModel sensor_model;

  [GtkChild (name = "treeview_sensors_list")]
  private Gtk.TreeView sensors_list;

  public string display_name {
	owned get { return "Sensors"; }
  }

  public string description {
	owned get { return "Sensors view"; }
  }

  public string id {
	owned get { return "/com/gaswarnanlagen/xmz/sensors"; }
  }


  construct {
    backend = new XMZ.DatabaseBackend ();
    // FIXME: Move init_sqlite () to DatabaseBackends contructor
    backend.init_sqlite ();
    sensors = backend.get_sensors ();

    sensor_model = new SensorModel (sensors);
    sensors_list.set_model (sensor_model);

    sensors_list.insert_column_with_attributes (-1, "ID", new Gtk.CellRendererText (), "text", 0);
    sensors_list.insert_column_with_attributes (-1, "Name", new Gtk.CellRendererText (), "text", 1);
    sensors_list.insert_column_with_attributes (-1, _("ADC Value"), new Gtk.CellRendererText (), "text", 2);

    sensors_list.set_hexpand (true);
  }

  private void on_selection (Gtk.TreeSelection selection) {
    Gtk.TreeModel model;
    Gtk.TreeIter iter;

  }

}
}
