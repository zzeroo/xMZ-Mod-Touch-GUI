using Modbus;

namespace XMZ {

public class TestModbusClient : Object {

  private Context context;

  public TestModbusClient (int server_id) {
    try {
      context = new Context.rtu ("/dev/ttyUSB0", 9600, 'N', 8, 1);
    } catch (Error e) {
      error ("Could not create modus context: %s", e.message);
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

  public static int main (string[] args) {
    var app = new TestModbusClient (1);

    message ("Modbus Client Test program started");

    return 0;
  }

}
}
