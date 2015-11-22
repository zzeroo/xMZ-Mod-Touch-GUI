
namespace XMZ {

public class SensorListBox : Gtk.ListBox {

  [GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-sensor-list-box-row.ui")]
  public class Row : Gtk.ListBoxRow {

	private Sensor? d_sensor;
	private DateTime d_time;

	[GtkChild]
    private Gtk.Image d_image;
	[GtkChild]
    private Gtk.Label d_sensor_label;
	[GtkChild]
    private Gtk.Label d_sensor_description_label;
	[GtkChild]
    private Gtk.Box d_sensor_values_box;
	[GtkChild]
    private Gtk.Arrow d_arrow;
	[GtkChild]
    private Gtk.Spinner d_spinner;

	private static Gtk.IconSize s_icon_size;

	static construct {
	  s_icon_size = Gtk.icon_size_register ("xmz", 64, 64);
	}

	public Sensor? sensor {
	  get { return d_sensor; }
	  set {
		d_sensor = value;
	  }
	}

	public string? sensor_name {
	  get { return d_sensor_label.get_text (); }
	  set { d_sensor_label.label = value; }
	}

	public Row (string name) {
	  Object (sensor_name: name);
	}

	construct {
	   show ();
	}
  }
}
}

// ex:ts=4 noet
