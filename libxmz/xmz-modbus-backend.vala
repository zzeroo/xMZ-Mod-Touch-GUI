namespace XMZ {

public class ModbusBackend : Object {

  public ModbusBackend () {

  }


  public uint16[] read_registers (uint16 modbus_address, uint16 start_register_address, uint16 size) {
    var ret_value = new uint16[size];

    return ret_value;
  }
}
}
