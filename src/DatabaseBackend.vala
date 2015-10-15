using GLib;
using Sqlite;
using Gee;


public class DatabaseBackend : GLib.Object {

  public Sqlite.Database database;
  private int error_code;
  private string error_msg;
  private const string prepared_query_string = "SELECT * FROM Sensors WHERE id = $UID;";
  private Sqlite.Statement statement;

  public DatabaseBackend () {
    error_code = Sqlite.Database.open ("database.db", out database);
    if (error_code != Sqlite.OK) {
      stderr.printf ("Can't open database: %d: %s\n", database.errcode (), database.errmsg ());
    }
  }

  ~DatabaseBackend () {
  }

  public int init_sqlite () {
    string query = """
        CREATE TABLE Sensors (
            id		INT		PRIMARY KEY		NOT NULL,
            name	TEXT					NOT NULL
            );

        INSERT INTO Sensors (id, name) VALUES (1, 'Sensor 1 (RA-GAS CO)');
        INSERT INTO Sensors (id, name) VALUES (2, 'Sensor 2 (RA-GAS CO/N02');
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
    error_code = database.prepare_v2 (prepared_query_string, prepared_query_string.length, out statement);
    if (error_code != Sqlite.OK) {
      error ("Error: %d: %s\n", error_code, error_msg);
    }

    int param_position = statement.bind_parameter_index ("$UID");
    assert (param_position > 0);

    statement.bind_int (param_position, 1);

    SensorNode sensor = new SensorNode (666, "Testsensor");

    return sensor;
  }

  public GenericArray<SensorNode> get_sensors () {
    GenericArray<SensorNode> data = new GenericArray<SensorNode> ();

    return data;
  }
}
