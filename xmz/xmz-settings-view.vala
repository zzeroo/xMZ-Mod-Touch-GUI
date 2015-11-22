
namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-settings-view.ui")]
class SettingsView : Gtk.Grid, XMZExt.UIElement, XMZExt.Activity {

  private const string version = Config.VERSION;
  public XMZExt.Application? application { owned get; construct set; }

  [GtkChild (name = "introduction")]
  private Gtk.Grid d_introduction;

  [GtkChild (name = "label_add")]
  private Gtk.Label d_label_add;

  [GtkChild (name = "scrolled_window")]
  private Gtk.ScrolledWindow d_scrolled_window;

  [GtkChild (name = "sensor_list_box")]
  private SensorListBox d_sensor_list_box;

  public bool has_sensors {
	get { return d_sensor_list_box.get_children ().length () != 0; }
  }

  public string display_name {
	owned get { return "Settings"; }
  }

  public string description {
	owned get { return "Settings view"; }
  }

  public string id {
	owned get { return "/com/gaswarnanlagen/xmz/settings"; }
  }

  construct {

	bind_property ("has_sensors",
				   d_scrolled_window,
				   "visible",
				   BindingFlags.SYNC_CREATE);

	bind_property ("has_sensors",
				   d_introduction,
				   "visible",
				   BindingFlags.SYNC_CREATE |
				   BindingFlags.INVERT_BOOLEAN);

	// Translators: the two %s will be replaced to create a link to perform the add-sensor action.
	d_label_add.label = _("Please %sadd a Sensor%s.").printf ("<a href=\"add-sensor\">", "</a>");
  }

  private void do_add_sensor (string name, bool report_errors) {
	//Sensor sensor;

	//try {
	  //sensor = new Sensor (name);
	//} catch (Error e) {
	   //application.show_infobar (_("Failed to add repository"), err.message, Gtk.MessageType.ERROR);
	//}
  }

  [GtkCallback]
  private bool test_message_activated () {
	application.show_infobar (_("Failed to add sensor"), _("Sensor could not be added."), Gtk.MessageType.INFO);

	return true;
  }

  [GtkCallback]
  private void add_sensor_clicked () {
	var dlg = new Gtk.MessageDialog (application as Gtk.Window,
									 Gtk.DialogFlags.MODAL,
									 Gtk.MessageType.WARNING,
									 Gtk.ButtonsType.OK_CANCEL,
									 _("Add Sensor"),
									 "foobar");

	var cancellable = new Cancellable ();

	dlg.response.connect (() => {
						   dlg.destroy ();
						  });
	dlg.show ();
  }


}
}

// ex:ts=4 noet
