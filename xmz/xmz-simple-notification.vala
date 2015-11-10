namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-simple-notification.ui")]
public class SimpleNotification : Gtk.Grid, XMZExt.Notification {

  // Do this to pull in config.h before glib.h (for gettext ...)
  private const string version = XMZ.Config.VERSION;

  [GtkChild (name = "spinner")]
  private Gtk.Spinner d_spinner;

  [GtkChild (name = "image_icon")]
  private Gtk.Image d_image_icon;

  [GtkChild (name = "label_title")]
  private Gtk.Label d_label_title;

  [GtkChild (name = "label_message")]
  private Gtk.Label d_label_message;

  [GtkChild (name = "button_cancel")]
  private Gtk.Button d_button_cancel;

  private bool d_finished;

  public signal void cancel ();

  public SimpleNotification (string? title = null, string? message = null) {
    Object (title: title, message: message);
    d_spinner.start ();
  }

  public Gtk.Widget? widget {
    owned get { return this; }
  }

  public void success (string message) {
    Idle.add(() => {
             d_spinner.stop ();
             d_spinner.hide ();

             d_image_icon.icon_name = "emblem-ok-symbolic";
             d_image_icon.show ();

             this.message = message;

             get_style_context ().add_class ("success");
             finish (true);

             return false;
             });
  }

  public void error (string message) {
    Idle.add(() => {
             d_spinner.stop ();
             d_spinner.hide ();

             d_image_icon.icon_name = "dialog-error-symbolic";
             d_image_icon.show ();

             this.message = message;

             get_style_context ().add_class ("error");
             finish (false);

             return false;
             });
  }

  private void finish (bool auto_close) {
    Idle.add (() => {
              d_finished = true;
              d_button_cancel.label = _("Close");

              if (auto_close) {
                close (3000);
              }

              return false;
              });
  }


  public string title {
    set {
      Idle.add(() => {
               d_label_title.label = value;
               return false;
               });
    }
  }

  public string message {
    set {
      Idle.add(() => {
               d_label_message.label = value;
               return false;
               });
     }
    }

    [GtkCallback]
    private void on_button_cancel_clicked () {
      if (d_finished) {
        close ();
      } else {
        cancel ();
      }
    }
  }

}
