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
    // TODO: This is the right place to update window title labels and so on
    base.present ();
  }

  private void setup_sensors_treeview () {
    // Model
    GenericArray<Sensor> data = new GenericArray<Sensor> ();
    data.add (new Sensor ("Sensor 1 CO", -1));
    data.add (new Sensor ("Sensor 1 NO²", -1));
    data.add (new Sensor ("Sensor 2 CO", -1));
    data.add (new Sensor ("Sensor 2 NO²", -1));
    data.add (new Sensor ("Sensor 3 CO", -1));
    data.add (new Sensor ("Sensor 3 NO²", -1));
    data.add (new Sensor ("Sensor 4 CO", -1));
    data.add (new Sensor ("Sensor 4 NO²", -1));
    data.add (new Sensor ("Sensor 5 CO", -1));
    data.add (new Sensor ("Sensor 5 NO²", -1));
    data.add (new Sensor ("Sensor 6 CO", -1));
    data.add (new Sensor ("Sensor 6 NO²", -1));

    SensorModel model = new SensorModel (data);
    // View
    sensors_treeview.set_rules_hint (true);
    sensors_treeview.set_model (model);

    var cell_name = new Gtk.CellRendererText ();
    cell_name.width = 300;
    var cell = new Gtk.CellRendererText ();

    sensors_treeview.insert_column_with_attributes (-1, _("Name"),      cell_name, "text", 0);
    sensors_treeview.insert_column_with_attributes (-1, _("ADC_Value"), cell, "text", 1);

    sensors_treeview.expand = true;
  }
}
}
