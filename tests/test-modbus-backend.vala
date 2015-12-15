/**
  */
namespace XMZ {

class TestModbusBackend : Gee.TestCase {

  public TestModbusBackend () {
    base ("TestModbusBackend");
    add_test ("uint16[] read_registers (uint modbus_address, uint start_add_register, uint size)", read_registers);
  }

  public override void set_up () {
    var modbus_backend = new ModbusBackend ();
  }

  public override void tear_down () {
  }


  public void read_registers () {

    assert (true);
  }
}
}
