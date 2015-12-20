/** CRUD
  * Create, Read, Update, Delete and Index actions
  */
namespace XMZ {

public class SensorController : Object {

  private ModbusBackend modbus_backend;
  private GenericArray<Sensor> sensors = new GenericArray<Sensor> ();

  private int serial_interface { get; set; }

  public SensorController () {
    modbus_backend = new ModbusBackend ();

    for (int i = 1; i < 7; i++) {
      sensors.add ( new Sensor ("Sensor "+ i.to_string () + " CO" , 0, i+40) );
      sensors.add ( new Sensor ("Sensor "+ i.to_string () + " NO²", 0, i+40) );
    }
  }

  ~SensorController () {
  }

  public Sensor get_sensor (int id) {
    if (id <= sensors.length) {
      return sensors[id];
    } else {
      error ("Invalid sensor id: %d\n", id);
    }
  }

  public GenericArray<Sensor> get_sensors () {

    return sensors;
  }


  public async void update_sensors () {
    new Thread<void*> (null, () => {
                       uint16[] response_register = {0};

                       while(true) {
                       sensors.foreach ((sensor) => {
                                        if (modbus_backend.read_registers ((uint16)sensor.modbus_address, 1, 1, out response_register) == 0) {
                                        sensor.adc_value = response_register[0];
                                        };
                                        Thread.usleep (100000);
                                        });
                        }
                        Idle.add (update_sensors.callback);

                       return null;
                       });
    yield;
  }


}
}
