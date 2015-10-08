using Gtk;

public class Application : Gtk.Window {
  public Application () {
    // Prepare Gtk.Window
    this.title = "xMZ-Mod-Touch";
    this.window_position = Gtk.WindowPosition.CENTER;
    this.set_resizable (false);
    this.set_deletable (false);
    this.set_hide_titlebar_when_maximized (true);
    if (GLib.Environment.get_variable ("XMZ_HARDWARE") == "0.1.0") {
      //this.set_decorated (false);
      this.maximize ();
    } else {
      this.set_default_size (1024, 600);
    }
  }

  public static int main (string[] args) {
    Gtk.init (ref args);

    Application app = new Application ();
    app.show_all ();
    Gtk.main ();
    return 0;
  }
}
