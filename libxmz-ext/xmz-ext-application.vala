
namespace XMZExt {

/**
 * Application is an interface to access the main xmz appliaction.
 *
 * The application interface is provided to plugins to access the main xmz
 * application instance. It contains prperties to access the currently open
 * repository as well as methods to open or create repositories.
 *
 */
public interface Application : Object {

//  public signal void repository_changed_externally (ExternalChangeHint hint);

  /**
   * An application wide message bus over which plugins can communicate
   */
//  public abstract XMZExt.MessageBus message_bus { owned get; }

  /**
   * The currrent application main activity.
   */
  public abstract XMZExt.Activity? current_activity { owned get; }


}
}
