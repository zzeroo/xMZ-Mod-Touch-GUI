using GLib;
using Sqlite;
using Gee;


public class DatabaseBackend : GLib.Object {

  public Sqlite.Database database;
  private int error_code;
  private string error_msg;
  private const string prepared_query_string = "SELECT * FROM Sensors WHERE id = $UID;";
  public static SensorNode sensor;
  public static GenericArray<SensorNode> sensors = new GenericArray<SensorNode> ();

  public DatabaseBackend () {
    // FIXME: Customize sqlite database path
    error_code = Sqlite.Database.open ("database.db", out database);
    if (error_code != Sqlite.OK) {
      stderr.printf ("Can't open database: %d: %s\n", database.errcode (), database.errmsg ());
    }
  }

  ~DatabaseBackend () {
  }

  private static int sensors_callback (int n_columns, string[] values, string[] column_names) {
    int id = 0;
    string name = "";
    int adc_value = 0;
    int adc_messgas = 0;
    int adc_nullgas = 0;

    for (int i = 0; i < n_columns; i++) {
      switch (column_names[i]) {
        case "id":
          id = int.parse (values[i]);
          break;
        case "name":
          name = values[i];
          break;
        case "adc_value":
          adc_value = int.parse (values[i]);
          break;
        case "adc_messgas":
          adc_messgas = int.parse (values[i]);
          break;
        case "adc_nullgas":
          adc_nullgas = int.parse (values[i]);
          break;
        default:
          assert_not_reached ();
      }
    }
    sensor = new SensorNode (id, name, adc_value, adc_messgas, adc_nullgas);
    sensors.add (sensor);

    return 0;
  }

  // FIXME: Checke database exist
  public int init_sqlite () {
    string query = """
        CREATE TABLE Sensors (
            id		INT		PRIMARY KEY		NOT NULL,
            name	TEXT					NOT NULL,
            adc_value   INT,
            adc_messgas INT,
            adc_nullgas INT
            );

        -- INSERT INTO Sensors (id, name, adc_value, adc_messgas, adc_nullgas) VALUES (1, 'Sensor 1', 0, 0, 0);
    """;
    error_code = database.exec (query, null, out error_msg);
    if (error_code != Sqlite.OK) {
      stderr.printf ("Error: %s\n", error_msg);
      return -1;
    }
      stdout.printf ("Database created.\n");

      return 0;
  }

  public int store_sensor (SensorNode sensor) {
    int id = sensor.id;
    string name = sensor.name;
    int adc_value = sensor.adc_value;
    int adc_messgas = sensor.adc_messgas;
    int adc_nullgas = sensor.adc_nullgas;
    string query = @"INSERT INTO sensors (id, name, adc_value, adc_messgas, adc_nullgas) VALUES ('$id', '$name', '$adc_value', '$adc_messgas', '$adc_nullgas');";
    error_code = database.exec (query, sensors_callback, out error_msg);
    if (error_code != Sqlite.OK) {
       error ("Error: %s\n", error_msg);
    }

    return 0;
  }

  public SensorNode get_sensor (int id) {
    string query = "SELECT * FROM sensors";
    error_code = database.exec (query, sensors_callback, out error_msg);
    if (error_code != Sqlite.OK) {
       error ("Error: %s\n", error_msg);
    }

    return sensors[id];
  }

  public GenericArray<SensorNode> get_sensors () {
    string query = "SELECT * FROM sensors";
    error_code = database.exec (query, sensors_callback, out error_msg);
    if (error_code != Sqlite.OK) {
       error ("Error: %s\n", error_msg);
    }
    return sensors;
  }
}
