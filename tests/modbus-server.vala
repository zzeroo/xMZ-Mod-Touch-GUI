using Modbus;

namespace XMZ {

public class TestModbusServer : Object {

  private Context context;
  int return_code;

  public TestModbusServer () {
    context = new Context.rtu ("/dev/ptm50", 9600, 'N', 8, 1);
    context.set_slave (200);
    context.set_debug (true);

    return_code = context.connect ();
    if (return_code == -1) {
      stderr.printf("Unable to connect %s\n", Modbus.strerror(errno));
      return;
    }
  }

  ~TestModbusServer () {
  }

  public int run (string[] args) {
    return 0;
  }

  public static int main (string[] args) {


    return 0;
  }

}
}
