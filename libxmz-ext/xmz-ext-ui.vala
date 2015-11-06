

namespace XMZ {

public class UI {
  public static Gee.HashMap<string, Object>? from_builder (string path, ...) {
    var builder = new Gtk.Builder ();

    try {
      builder.add_from_resource ("/com/gaswarnanlagen/xmz/" + path);
    } catch (Error e) {
      warning ("Failed to load ui: %s", e.message);
      return null;
    }

    Gee.HashMap<string, Object> ret = new Gee.HashMap<string, Object>();

    var l = va_list ();

    while (true) {
      string? id = l.arg();

      if (id == null) {
        break;
      }

      ret[id] = builder.get_object (id);
    }

    return ret;
  }
}
}
