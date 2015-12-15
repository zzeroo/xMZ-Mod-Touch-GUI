using Modbus;

namespace XMZ {

public class TestModbusClient : Object {

  private Context context;
  private uint16[] response_register = new uint16[20];
  private int return_code;

  public TestModbusClient (int server_id) {
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

    context.set_slave (server_id);
  }

  ~TestModbusClient () {
    context.close ();
  }

  construct {
  }

  public int run () {
    if (context.connect () == -1) {
      stderr.printf ("Connection failed: %s\n", Modbus.strerror(errno));
      context.close ();
      return -1;
    }
    return_code = context.read_registers (0, 20, response_register);
    if (return_code != -1) {
      for (int i=0; i < 20; i++) {
        stdout.printf ("%d: %d\n", i, response_register[i]);
      }
    }

    return 0;
  }

  public static int main (string[] args) {
    int64 modbus_address;
    bool res = int64.try_parse (args[1], out modbus_address);

    if (res) {
      var app = new TestModbusClient ((int)modbus_address);
      app.run ();
    }


    return 0;
  }

}
}
