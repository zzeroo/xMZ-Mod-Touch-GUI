public class XMZ.Resource {

  public static Gtk.CssProvider? load_css (string id) {
    var provider = new Gtk.CssProvider ();
    var f = File.new_for_uri ("resource:///com/gaswarnanlagen/xmz/ui" + id);

    try {
      provider.load_from_file (f);
    } catch (Error e) {
      warning ("Error while loading resource: %s", e.message);
      return null;
    }

    return provider;
  }
}
