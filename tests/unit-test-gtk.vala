void add_foo_tests () {
  Test.add_func ("/vala/test", () => {
                 var widget = new Gtk.Button ();
                 assert (widget is Gtk.Button);
                 });
}

void main (string [] args) {
  Gtk.init (ref args);
  Test.init (ref args);

  add_foo_tests ();

  Idle.add (() => {
            Test.run ();
            Gtk.main_quit ();
            return false;
            });

  Gtk.main ();
}
