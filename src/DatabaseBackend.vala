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

    for (int i = 0; i < n_columns; i++) {
      switch (column_names[i]) {
        case "id":
          id = int.parse (values[i]);
          break;
        case "name":
          name = values[i];
          break;
        default:
          assert_not_reached ();
      }
    }
    sensor = new SensorNode (id, name);
    sensors.add (sensor);

    return 0;
  }

  public int init_sqlite () {
    string query = """
        CREATE TABLE Sensors (
            id		INT		PRIMARY KEY		NOT NULL,
            name	TEXT					NOT NULL
            );

        INSERT INTO Sensors (id, name) VALUES (1, 'Sensor 1');
        INSERT INTO Sensors (id, name) VALUES (2, 'Sensor 2');
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
    return 0;
  }

  public SensorNode get_sensor (int id) {
    string query = "SELECT * FROM sensors";
    error_code = database.exec (query, sensors_callback, out error_msg);
    if (error_code != Sqlite.OK) {
       error ("Error: %s\n", error_msg);
    }

    return sensors[id - 1];
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
