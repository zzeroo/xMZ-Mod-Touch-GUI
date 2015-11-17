

namespace XMZ {

public class DatabaseBackend : Object {

  private Sqlite.Database database;
  private int error_code;
  private string error_msg;


  construct {
  }

  public DatabaseBackend () {
    string path = XMZ.AppDirs.get_data_subdir ("data").get_child ("xmz.db").get_path ();
    error_code = Sqlite.Database.open (path, out database);
    if (error_code != Sqlite.OK) {
      stdout.printf (_("Sqlite Database could not be opened.\n"));
    }
  }

  public int init_sqlite () {
    string query = """
        CREATE TABLE Sensors (
            id          INT   PRIMARY KEY   NOT NULL,
            name        TEXT                NOT NULL,
            adc_value   INT,
            adc_at_nullgas  INT,
            adc_at_messgas  INT
                             );

        INSERT INTO Sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (1, 'Sensor 1', 0, 0, 0);
        """;

    error_code = database.exec (query, null, out error_msg);
    if (error_code != Sqlite.OK) {
      stderr.printf (_("Error: %s\n"), error_msg);
      return -1;
    }
    stdout.printf (_("Database successfully created.\n"));

    return 0;
  }

  public GenericArray<XMZ.Sensor> get_sensors () {
    var sensor = new XMZ.Sensor ("CO/NO KLS");
    var sensors = new GenericArray<XMZ.Sensor> ();

    sensors.add (sensor);

    return sensors;
  }

}
}
