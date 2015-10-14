void main (string[] args) {
  Test.init (ref args);

  TestSuite.get_root ().add_suite (new TestExample ().get_suite ());
  TestSuite.get_root ().add_suite (new TestSensorModel ().get_suite ());

	Test.run ();
}

