

namespace XMZ {

public class DatabaseBackend : Object {

  private Sqlite.Database database;
  private int error_code;
  private string error_msg;
  private const string prepared_query_srting = "SELECT * FROM Sensors WHERE id = $UID;";
  public static Sensor sensor;
  public static GenericArray<Sensor> sensors = new GenericArray<Sensor> ();


  public DatabaseBackend () {
    string path = XMZ.AppDirs.get_data_subdir ("data").get_child ("xmz.db").get_path ();
    error_code = Sqlite.Database.open (path, out database);
    if (error_code != Sqlite.OK) {
      // Translators: The %d and %s are for database error code and error message.
      stdout.printf (_("Can't open database: %d: %s\n"), database.errcode (), database.errmsg ());
    }
  }

  ~DatabaseBackend () {
  }

  // FIXME: Debug this function to get more about her function
  private static int sensors_callback (int n_columns, string[] values, string[] column_names) {
    int id = 0;
    string name = "";
    int adc_value = 0;
    int adc_at_nullgas = 0;
    int adc_at_messgas = 0;

    for (int i = 0; i < n_columns; i++) {
      switch (column_names [i]) {
        case "id":
          id = int.parse (values[i]);
          break;
        case "name":
          name = values[i];
          break;
        case "adc_value":
          adc_value = int.parse (values[i]);
          break;
        case "adc_at_nullgas":
          adc_at_nullgas = int.parse (values[i]);
          break;
        case "adc_at_messgas":
          adc_at_messgas = int.parse (values[i]);
          break;
        default:
          assert_not_reached ();
      }
    }
    sensor = new Sensor (id, name, adc_value, adc_at_nullgas, adc_at_messgas);
    sensors.add (sensor);

    return 0;
  }

  public int init_sqlite () {
    string query = """
        CREATE TABLE sensors (
            id              INT   PRIMARY KEY   NOT NULL,
            name            TEXT                NOT NULL,
            adc_value       INT,
            adc_at_nullgas  INT,
            adc_at_messgas  INT
                             );

        INSERT INTO sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (1, 'Sensor 1', 0, 0, 0);
        INSERT INTO sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (2, 'Sensor 2', 0, 0, 0);
        INSERT INTO sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (3, 'Sensor 3', 0, 0, 0);
        INSERT INTO sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (4, 'Sensor 4', 0, 0, 0);
        INSERT INTO sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (5, 'Sensor 5', 0, 0, 0);
        """;

    error_code = database.exec (query, null, out error_msg);
    if (error_code != Sqlite.OK) {
      stderr.printf (_("Error: %s\n"), error_msg);
      return -1;
    }
    stdout.printf (_("Database successfully created.\n"));

    return 0;
  }

  public int store_sensor (Sensor sensor) {
    int id = sensor.id;
    string name = sensor.name;
    int adc_value = sensor.adc_value;
    int adc_at_nullgas = sensor.adc_at_nullgas;
    int adc_at_messgas = sensor.adc_at_messgas;
    string query = @"INSERT INTO sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES ('$id', '$name', '$adc_value', '$adc_at_nullgas', '$adc_at_messgas');";
    error_code = database.exec (query, sensors_callback, out error_msg);
    if (error_code != Sqlite.OK) {
      error ("Error: %s\n", error_msg);
    }

    return 0;
  }

  public Sensor get_sensor (int id) {
    string query = "SELECT * FROM sensors";
    error_code = database.exec (query, sensors_callback, out error_msg);
    if (error_code != Sqlite.OK) {
      error ("Error: %s\n", error_msg);
    }

    return sensors[id];
  }

  public GenericArray<Sensor> get_sensors () {
    string query = "SELECT * FROM sensors";
    error_code = database.exec (query, sensors_callback, out error_msg);
    if (error_code != Sqlite.OK) {
      error ("Error: %s\n", error_msg);
    }

    return sensors;
  }

}
}
