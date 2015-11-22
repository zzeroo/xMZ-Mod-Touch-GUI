
namespace XMZ {

public class Notifications : Object, XMZExt.Notifications {

  private Gtk.Overlay d_overlay;
  private Gee.HashMap<XMZExt.Notification, uint> d_delay_handles;
  private Gtk.Box d_box;
  private Gee.HashMap<XMZExt.Notification, ulong> d_handles;

  public Notifications (Gtk.Overlay overlay) {
    d_overlay = overlay;
    d_delay_handles = new Gee.HashMap<XMZExt.Notification, uint> ();
    d_handles = new Gee.HashMap<XMZExt.Notification, ulong> ();

    d_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 3);
    d_box.get_style_context ().add_class ("notifications");
    d_box.show ();

    d_box.valign = Gtk.Align.END;
    d_overlay.add_overlay (d_box);
  }

  public override void dispose () {
    foreach (var id in d_delay_handles.values) {
      Source.remove (id);
    }

    d_delay_handles.clear ();

    foreach (var notification in d_handles.keys) {
      notification.disconnect (d_handles[notification]);
    }

    d_handles.clear ();

    base.dispose ();
  }

  public new void add (XMZExt.Notification notification) {
    var revealer = new Gtk.Revealer ();

    revealer.margin_top = 1;
    revealer.set_transition_duration (500);
    revealer.set_transition_type (Gtk.RevealerTransitionType.SLIDE_UP);
    revealer.add (notification.widget);

    notification.widget.show ();
    revealer.show ();

    d_box.add (revealer);
    revealer.reveal_child = true;

    d_handles[notification] = notification.close.connect((delay) => {
                                                          remove (notification, delay);
                                                         });
  }

  private void remove_now (XMZExt.Notification notification) {
    var revealer = notification.widget.get_parent () as Gtk.Revealer;

    notification.disconnect (d_handles[notification]);

    revealer.notify["child-revealed"].connect (() => {
                                                revealer.remove (notification.widget);
                                                revealer.destroy ();
                                               });

    revealer.reveal_child = false;
  }

  public void remove (XMZExt.Notification notification, uint delay) {
    if (d_delay_handles.has_key (notification)) {
      Source.remove (d_delay_handles[notification]);
    }

    d_delay_handles[notification] = Timeout.add (delay, () => {
                                                 d_delay_handles.unset (notification);
                                                 remove_now (notification);

                                                 return false;
                                                 });
  }
}
}

// ex:set ts=4 noet:
