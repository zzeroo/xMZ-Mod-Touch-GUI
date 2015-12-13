/**
  */
namespace XMZ {

class TestModbusFunctions : Gee.TestCase {

  public TestModbusFunctions() {
    base ("TestModbusFunctions");
    add_test ("test_example", test_example);
    add_test ("test another example", test_another_example);
  }

  public override void set_up () {
  }

  public override void tear_down () {
  }


  public void test_example () {
    assert (true);
  }

  public void test_another_example () {
    assert (true);
  }
}
}
