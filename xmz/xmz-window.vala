namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-window.ui")]
public class Window : Gtk.ApplicationWindow {

  // Do this to pull in config.h before glib.h (for gettext)
  private const string version = XMZ.Config.VERSION;

  [GtkChild]
  private Gtk.Overlay overlay;
  [GtkChild]
  private Gtk.Grid main_grid;
  [GtkChild]
  private Gtk.Stack main_stack;
  [GtkChild]
  private Gtk.Grid sensors_list_grid;
  [GtkChild]
  private Gtk.TreeView sensors_treeview;

  // TODO: factore out SensorsModel, SensorsController and
  // SensorsModel[] sensors = SensorsController.sensors (),
  // SensorsModel   sensor  = SensorsController.sensor (1)
  private SensorModel[] sensors = {
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
    new SensorModel ("Sensor CO"),
    new SensorModel ("Sensor NO²"),
  };

  private Gdk.Geometry hints;


  construct {
  }

  private bool init () {
    if (GLib.Environment.get_variable ("XMZ_HARDWARE") == "0.1.0") {
      set_deletable (false);
      set_hide_titlebar_when_maximized (true);
      maximize ();
    } else {
      set_default_size (1024, 600);
      set_resizable (false);
      hints.min_width = -1;
      hints.max_width = 1024;
      hints.min_height = -1;
      hints.max_height = 600;
      set_geometry_hints(this, hints, Gdk.WindowHints.MIN_SIZE | Gdk.WindowHints.MAX_SIZE);
    }

    setup_sensors_treeview ();

    main_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
    main_stack.set_visible_child (sensors_list_grid);

    return true;
  }

  public static Window? create_new (Gtk.Application app) {
    Window? ret = new XMZ.Window ();

    if (ret != null) {
      ret.application = app;
    }

    try {
      ret.init ();
    } catch {}

    return ret;
  }

  public new void present () {
    base.present ();
  }

  private void setup_sensors_treeview () {
    var sensor_list_model = new Gtk.ListStore (2, typeof (string), typeof (int));
    sensors_treeview.set_model (sensor_list_model);
    sensors_treeview.insert_column_with_attributes (-1, _( "Name" ),
                                        new Gtk.CellRendererText (), "text",
                                        0);

    sensors_treeview.insert_column_with_attributes (-1, _( "ADC_Value" ),
                                        new Gtk.CellRendererText (), "text",
                                        1);

    Gtk.TreeIter iter;
    for (int i = 0; i < sensors.length; i++) {
      sensor_list_model.append (out iter);
      sensor_list_model.set (iter,
                             0, sensors[i].name,
                             1, sensors[i].adc_value);
    }

    sensors_treeview.expand = true;
  }
}
}
