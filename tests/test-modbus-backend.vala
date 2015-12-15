/**
  */
namespace XMZ {

class TestModbusBackend : Gee.TestCase {

  private ModbusBackend modbus_backend;

  public TestModbusBackend () {
    base ("TestModbusBackend");
    add_test ("int read_registers (int modbus_address, uint16 start_add_register, uint size, out uint16[] response_register)", read_registers);
  }

  public override void set_up () {
    modbus_backend = new ModbusBackend ();
  }

  public override void tear_down () {
  }


  public void read_registers () {
    uint16[] response_register;
    modbus_backend.read_registers (16, 1, 1, out response_register);

    assert (response_register.length == 1);
  }
}
}
