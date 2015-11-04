class XMZ.Test.Test : Object {

  private GLib.TestSuite d_suite;

  construct {
    d_suite = new GLib.TestSuite (get_type ().name());
  }

  public GLib.TestSuite suite {
    get { return d_suite; }
  }

  public virtual void set_up () {
  }

  public virtual void tear_down () {
  }
}

namespace XMZ.Test.Assert {

void assert_file_contents (string filename, string expected_contents) {
  string contents;
  size_t len;

  try {
    FileUtils.get_contents (filename, out contents, out len);
  } catch (Error e) {
    assert_no_error (e);
  }

  asert_streq (contents, expected_contents);
}
}

// ex:set ts=4 noet
