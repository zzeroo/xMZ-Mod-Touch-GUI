using Gtk;

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz-mod-touch/sensorwidget.glade")]
public class SensorWidget : Stack {
				[GtkChild]
				private LevelBar levelbar;

				[GtkChild]
				private Label lbl_value;
}

void main(string[] args) {
				Gtk.init (ref args);
				var window = new Window ();
				window.destroy.connect (Gtk.main_quit);

				var sensor1 = new SensorWidget ();


				window.add (sensor1);
				window.show_all ();

				Gtk.main ();
}

