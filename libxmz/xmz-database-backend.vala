

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
            id              INT   PRIMARY KEY   NOT NULL,
            name            TEXT                NOT NULL,
            adc_value       INT,
            adc_at_nullgas  INT,
            adc_at_messgas  INT
                             );

        INSERT INTO Sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (1, 'Sensor 1', 0, 0, 0);
        INSERT INTO Sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (2, 'Sensor 2', 0, 0, 0);
        INSERT INTO Sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (3, 'Sensor 3', 0, 0, 0);
        INSERT INTO Sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (4, 'Sensor 4', 0, 0, 0);
        INSERT INTO Sensors (id, name, adc_value, adc_at_nullgas, adc_at_messgas) VALUES (5, 'Sensor 5', 0, 0, 0);
        """;

    error_code = database.exec (query, null, out error_msg);
    if (error_code != Sqlite.OK) {
      stderr.printf (_("Error: %s\n"), error_msg);
      return -1;
    }
    stdout.printf (_("Database successfully created.\n"));

    return 0;
  }

  public GenericArray<Sensor> get_sensors () {
    var sensors = new GenericArray<Sensor> ();

    for (int i = 0; i < 10; i++) {
      var sensor = new XMZ.Sensor (i, "CO/NO Kombisensor RA-Gas GmbH", 0, 0, 0);
      sensors.add (sensor);
    }
    return sensors;
  }

}
}
