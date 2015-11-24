


class XMZ.Test.Main {

  private TestCase[] d_cases;

  public Main (string [] args) {
    //XMZ.init ();
    GLib.Test.init (ref args);
  }

  public void run () {
    GLib.Test.run ();
  }

  class TestCase : Object {
    private XMZ.Test.Test d_test;
    private string d_name;
    private uint d_signal_id;

    public TestCase (XMZ.Test.Test test, string name, uint signal_id) {
      d_test = test;
      d_name = name;
      d_signal_id = signal_id;
    }

    public string name {
      get {return d_name; }
    }

    public void set_up() {
      d_test.set_up ();
    }

    public void tear_down() {
      d_test.tear_down ();
    }

    public void test() {
      GLib.Signal.emit (d_test, d_signal_id, 0);
    }
  }

  public void add (...) {
    var l = va_list ();
    Test test;

    while ((test = l.arg<Test>()) != null) {
      add_test (test);
    }
  }

  public void add_test (Test test) {
    var ids = GLib.Signal.list_ids (test.get_type ());

    var suite = test.suite;

    GLib.TestSuite.get_root ().add_suite (suite);

    foreach (var id in ids) {
      GLib.SignalQuery q;

      GLib.Signal.query (id, out q);

      if (q.n_params != 0 || !q.signal_name.has_prefix ("test-")) {
        continue;
      }

      var c = new TestCase (test,
                            q.signal_name[5:q.signal_name.length],
                            q.signal_id);

      d_cases += c;

      var tc = new GLib.TestCase (c.name,
                                  c.set_up,
                                  c.test,
                                  c.tear_down);

      suite.add (tc);
    }
  }
}

// ex:set ts=4 noet
