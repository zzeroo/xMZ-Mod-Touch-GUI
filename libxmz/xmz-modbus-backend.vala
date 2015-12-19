using Modbus;

namespace XMZ {

public class ModbusBackend : Object {

  private Context context;
  private int return_code;
  private uint16[] response_register = new uint16[20];

  public ModbusBackend () {
    if (GLib.Environment.get_variable ("XMZ_HARDWARE") == "0.1.0") {
      context = new Context.rtu ("/dev/ttyS1", 9600, 'N', 8, 1);
      context.rtu_set_serial_mode (ModbusRTU.RS485);
      context.rtu_set_rts (ModbusRTU.RTS_DOWN);
    } else {
      context = new Context.rtu ("/dev/ttyUSB0", 9600, 'N', 8, 1);
    }

    context.set_debug (true);
    context.set_error_recovery (ModbusError.RECOVERY_LINK |
                                ModbusError.RECOVERY_PROTOCOL);
  }

  ~ModbusBackend() {
    context.close ();
  }


  public int read_registers (uint16 modbus_address, uint16 start_register_address, uint16 size, out uint16[] response_register) {
    response_register = {0};
    context.set_slave (modbus_address);
    if (context.connect () == -1) {
      message ("Connection failed: %s\n", Modbus.strerror (errno));
      context.close ();
      return -1;
    }
    return_code = context.read_registers (start_register_address, size, response_register);
    if (return_code != 1) {
      message ("Error read_registers, return_code: %d\n", return_code);
      return 1;
    } else {
      return 0;
    }
  }
}
}
