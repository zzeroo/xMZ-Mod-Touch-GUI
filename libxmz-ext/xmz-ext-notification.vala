
namespace XMZExt {

public interface Notification : Object {
  public signal void close (uint delay = 0);
  public abstract Gtk.Widget? widget { owned get;  }
}
}

// ex:set ts=4 noet:
