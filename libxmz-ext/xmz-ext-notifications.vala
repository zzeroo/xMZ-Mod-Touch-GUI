
namespace XMZExt {

public interface Notifications : Object {
  public abstract void add (Notification notification);
  public abstract void remove (Notification notification, uint delay);
}
}

// ex:set ts=4 noet:
