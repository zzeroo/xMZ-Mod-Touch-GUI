namespace XMZ {

[GtkTemplate (ui = "/com/gaswarnanlagen/xmz/ui/xmz-window.ui")]
public class Window : Gtk.ApplicationWindow {

  [GtkChild]
  private Gtk.Stack main_stack;
  [GtkChild]
  private Gtk.Grid main_grid;
  [GtkChild]
  private Gtk.Grid sensors_list;


  construct {
    assert (true);
  }

  private bool init () {
    if (GLib.Environment.get_variable ("XMZ_HARDWARE") == "0.1.0") {
      set_deletable (false);
      set_hide_titlebar_when_maximized (true);
      maximize ();
    } else {
      set_default_size (1024, 600);
    }
    main_stack.transition_type = Gtk.StackTransitionType.SLIDE_LEFT;
    main_stack.set_visible_child (sensors_list);

    return true;
  }

  public static Window? create_new (Gtk.Application app) {
    Window? ret = new Window ();

    if (ret != null) {
      ret.application = app;
    }

    return ret;
  }

  public new void present () {
    base.present ();
  }
}
}
