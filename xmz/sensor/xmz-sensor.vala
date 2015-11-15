
namespace XMZSensor {

public class Activity : Object, XMZExt.UIElement, XMZExt.Activity {

  // Do this to pull in config.h before glib.h (for gettext...)
  private const string version = XMZ.Config.VERSION;

  public XMZExt.Application? application { owned get; construct set; }

  public string id {
	owned get { return "/com/gaswarnanlagen/xmz/sensor"; }
  }

  construct {}


  public string display_name {
	owned get { return C_("Activity", "Sensor"); }
  }

  public string description {
	owned get { return _("Create a new sensor"); }
  }
}
}
// ex: ts=4 noet
