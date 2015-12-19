namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-window.ui")]
public class Window : Gtk.ApplicationWindow {

  // Do this to pull in config.h before glib.h (for gettext)
  private const string version = XMZ.Config.VERSION;

  //[GtkChild]
  //private Gtk.Overlay overlay;
  //[GtkChild]
  //private Gtk.Grid main_grid;
  [GtkChild]
  private Gtk.Stack main_stack;
  [GtkChild]
  private Gtk.Grid sensors_list_grid;
  [GtkChild]
  private Gtk.TreeView sensors_treeview;
  private SensorController sensor_controller;
  private GenericArray<Sensor> sensors;
  private SensorModel model;
  private Gdk.Geometry hints;


  construct {
    sensor_controller = new SensorController ();
    sensors = sensor_controller.get_sensors ();
    model = new SensorModel (sensors);
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
    Thread.create<void> (sensor_controller.update_sensors, false);
    main_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
    main_stack.set_visible_child (sensors_list_grid);

    return true;
  }

  public static Window? create_new (Gtk.Application app) {
    Window? ret = new XMZ.Window ();

    if (ret != null) {
      ret.application = app;
    }
    ret.init ();

    return ret;
  }

  public new void present () {
    // TODO: This is the right place to update window title labels and so on
    base.present ();
  }

  private void setup_sensors_treeview () {
    // View
    sensors_treeview.set_rules_hint (true);
    sensors_treeview.set_model (model);

    var cell_name = new Gtk.CellRendererText ();
    cell_name.width = 300;
    var cell = new Gtk.CellRendererText ();

    sensors_treeview.insert_column_with_attributes (-1, _("Name"),      cell_name, "text", 0);
    sensors_treeview.insert_column_with_attributes (-1, _("ADC_Value"), cell, "text", 1);
    sensors_treeview.insert_column_with_attributes (-1, _("Volt"), cell, "text", 2);
    sensors_treeview.insert_column_with_attributes (-1, _("Value"), cell, "text", 3);
    sensors_treeview.insert_column_with_attributes (-1, _("SI"), cell, "text", 4);

    sensors_treeview.expand = true;
  }
}
}
