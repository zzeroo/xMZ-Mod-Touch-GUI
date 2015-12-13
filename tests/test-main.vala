/**
  */
namespace XMZ {

public static int main (string[] args) {
  Test.init (ref args);
  Gtk.init (ref args);

  TestSuite.get_root ().add_suite (new TestModbusFunctions ().get_suite ());
  TestSuite.get_root ().add_suite (new TestSensorController ().get_suite ());

  return Test.run ();
}
}
