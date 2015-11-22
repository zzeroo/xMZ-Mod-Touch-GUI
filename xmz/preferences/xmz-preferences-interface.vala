namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-preferences-interface.ui")]
public class PreferencesInterface : Gtk.Grid, XMZExt.Preferences {

  // Do this to pull in config.h before glib.h (for gettext ...)
  private const string version = XMZ.Config.VERSION;
  private Settings? d_settings;

  construct {
    d_settings = new Settings ("com.gaswarnanlagen.xmz.preferences.interface");
  }

  public Gtk.Widget widget {
    owned get {
      return this;
    }
  }

  public string id {
    owned get { return "/com/gaswarnanlagen/xmz/Preferences/Interface"; }
  }

  public string display_name {
    owned get { return _("Interface"); }
  }
}
}
