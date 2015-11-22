

namespace XMZExt {

public interface Preferences : Object {
  public abstract string id { owned get; }
  public abstract string display_name { owned get; }
  public abstract Gtk.Widget widget { owned get; }
}
}

// vi:ts=4
