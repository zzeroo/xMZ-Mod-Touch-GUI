using Gtk;

public class Application : Gtk.Window {

  private DatabaseBackend backend;

  public Application () {
    backend = new DatabaseBackend ();
    backend.init_sqlite ();

    // Prepare Gtk.Window
    this.title = "xMZ-Mod-Touch";
    this.window_position = Gtk.WindowPosition.CENTER;
    if (GLib.Environment.get_variable ("XMZ_HARDWARE") == "0.1.0") {
      // this.set_decorated (false);
      // this.set_resizable (false); // window is dame small under weston if used
      this.set_deletable (false);
      this.set_hide_titlebar_when_maximized (true);
      this.maximize ();
    } else {
      this.set_default_size (1024, 600);
    }

    var builder = new Builder ();
    //builder.add_from_file ("src/MainWindow.glade");
    try {
      builder.add_from_resource ("/com/gaswarnanlagen/xmz-mod-touch-gui/xmz-mod-touch-gui.glade");
    } catch (Error e) {
      error ("%s", e.message);
    }

    builder.connect_signals (this);

    var box = builder.get_object ("box") as Box;
    this.add (box);

    var menu = builder.get_object ("menu") as ButtonBox;
    box.pack_start (menu);

    var stack = builder.get_object ("stack_main_window") as Stack;
    box.pack_end (stack);

    var settings_button = builder.get_object ("button_settings") as Button;
    settings_button.clicked.connect(() => {
                                    stdout.printf ("STACK: %s", stack.get_visible_child_name ());
                                      switch (stack.get_visible_child_name ()) {
                                        case "page0":
                                          stack.set_visible_child_name ("page1");
                                          break;
                                        case "page1":
                                          stack.set_visible_child_name ("page0");
                                          break;
                                        default:
                                          stack.set_visible_child_name ("page0");
                                          break;
                                      }
                                    });

    // Sensor Model
    GenericArray<SensorNode> sensors = new GenericArray<SensorNode> ();
    sensors = backend.get_sensors ();
    SensorModel model = new SensorModel (sensors);

    // View
    Gtk.TreeView view = new Gtk.TreeView.with_model (model);
    view.insert_column_with_attributes (-1, "ID",   new Gtk.CellRendererText (), "text", 0);
    view.insert_column_with_attributes (-1, "Name", new Gtk.CellRendererText (), "text", 1);

    var grid_main_window = builder.get_object ("grid_main_window") as Grid;
    view.set_hexpand (true);
    grid_main_window.attach (view, 0, 0, 3, 3);
  }

  public static int main (string[] args) {
    Gtk.init (ref args);

    Application app = new Application ();
    app.show_all ();
    Gtk.main ();
    return 0;
  }
}
