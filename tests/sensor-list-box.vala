using Gtk;
using XMZ;

class TestSensorListBox
{
	public static int main(string[] args)
	{
		Gtk.init(ref args);

		try
		{
			//XMZ.init();
		}
		catch (Error e)
		{
			stderr.printf("Failed to initialize ggit: %s\n", e.message);
			return 1;
		}

		var window = new Window();
		window.set_default_size(300, 300);
		window.add(new SensorListBox());
		window.show_all();

		window.delete_event.connect((w, ev) => {
			Gtk.main_quit();
			return true;
		});

		Gtk.main();

		return 0;
	}
}

// vi:ts=4

