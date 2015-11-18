

namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-sensor-view.ui")]
class SensorView : Gtk.Stack, XMZExt.UIElement, XMZExt.Activity {

  private const string version = Config.VERSION;
  public XMZExt.Application? application { owned get; construct set; }

  private XMZ.DatabaseBackend d_backend;

  private GenericArray<Sensor> d_sensors;
  private XMZ.SensorModel d_sensor_model;

  [GtkChild (name = "treeview_sensors_list")]
  private Gtk.TreeView d_sensors_list;


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
    d_backend = new XMZ.DatabaseBackend ();
    d_backend.init_sqlite ();

    d_sensors = d_backend.get_sensors ();

    d_sensor_model = new SensorModel (d_sensors);

    d_sensors_list.set_model (d_sensor_model);
    d_sensors_list.insert_column_with_attributes (-1, "ID", new Gtk.CellRendererText (), "text", 0);
    d_sensors_list.insert_column_with_attributes (-1, "Name", new Gtk.CellRendererText (), "text", 1);
    d_sensors_list.insert_column_with_attributes (-1, _("ADC Value"), new Gtk.CellRendererText (), "text", 2);

    d_sensors_list.show_all ();

    stdout.printf (d_sensors[0].name + "\n");
  }

}
}
