
namespace XMZExt {

/**
 * Application is an interface to access the main xmz application.
 *
 * The application interface is provided to plugins to access the main xmz
 * application instance. It contains properties to access the currently open
 * repository as well as methods to open or create repositories.
 *
 */
public interface Application : Object {

  /**
   * The currrent application main activity.
   */
  public abstract XMZExt.Activity? current_activity { owned get; }

  /**
   * Fet the notifications manager for the application.
   */
  public abstract Notifications notifications { owned get; }

  public abstract void show_infobar (string primary_msg,
                                    string secondary_msg,
                                    Gtk.MessageType type);
}
}

// ex:set ts=4 noet:
