using Gtk;

int main (string[] args) {
    Gtk.init (ref args);
    Gtk.Settings.get_default().set("gtk-application-prefer-dark-theme", true);
    Gtk.Settings.get_default().set("gtk-font-name", "DejaVu Sans 18");

    var window = new Window ();
    window.title = "xMZ-Mod-Touch x-Messzentrale";
    window.border_width = 10;
    window.set_default_size (1024, 600);
    window.maximize ();
    window.destroy.connect (Gtk.main_quit);
    var entry = new Entry ();
    window.add (entry);
    window.show_all ();

    Gtk.main ();
    return 0;
}
