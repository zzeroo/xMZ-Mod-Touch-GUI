using Gtk;

public class Application : Gtk.Window {
  [CCode (instance_pos=-1)]
  public void on_button_settings_clicked (Button source) {
    stdout.printf ("function: on_button_settings_clicked\n");
  }

  public Application () {
    // Prepare Gtk.Window
    this.title = "xMZ-Mod-Touch";
    this.window_position = Gtk.WindowPosition.CENTER;
    if (GLib.Environment.get_variable ("XMZ_HARDWARE") == "0.1.0") {
      //this.set_decorated (false);
      this.set_resizable (false);
      this.set_deletable (false);
      this.set_hide_titlebar_when_maximized (true);
      this.maximize ();
    } else {
      this.set_default_size (1024, 600);
    }

    var builder = new Builder ();
    builder.add_from_file ("src/MainWindow.glade");

    builder.connect_signals (this);

    var box = builder.get_object ("box") as Box;
    this.add (box);

    var menu = builder.get_object ("menu") as ButtonBox;
    box.pack_start (menu);

    var stack = builder.get_object ("stack_window") as Stack;
    box.pack_end (stack);

  }

  public static int main (string[] args) {
    Gtk.init (ref args);

    Application app = new Application ();
    app.show_all ();
    Gtk.main ();
    return 0;
  }
}
