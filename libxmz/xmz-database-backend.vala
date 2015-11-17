

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
            ID              INT   PRIMARY KEY   NOT NULL,
            NAME            TEXT                NOT NULL,
            ADC_VALUE       INT,
            ADC_AT_NULLGAS  INT,
            ADC_AT_MESSGAS  INT
                             );

        INSERT INTO Sensors (ID, NAME, ADC_VALUE, ADC_AT_NULLGAS, ADC_AT_MESSGAS) VALUES (1, 'Sensor 1', 0, 0, 0);
        INSERT INTO Sensors (ID, NAME, ADC_VALUE, ADC_AT_NULLGAS, ADC_AT_MESSGAS) VALUES (2, 'Sensor 2', 0, 0, 0);
        INSERT INTO Sensors (ID, NAME, ADC_VALUE, ADC_AT_NULLGAS, ADC_AT_MESSGAS) VALUES (3, 'Sensor 3', 0, 0, 0);
        INSERT INTO Sensors (ID, NAME, ADC_VALUE, ADC_AT_NULLGAS, ADC_AT_MESSGAS) VALUES (4, 'Sensor 4', 0, 0, 0);
        INSERT INTO Sensors (ID, NAME, ADC_VALUE, ADC_AT_NULLGAS, ADC_AT_MESSGAS) VALUES (5, 'Sensor 5', 0, 0, 0);
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
    var sensors = new GenericArray<XMZ.Sensor> ();

    var sensor = new XMZ.Sensor (1, "CO/NO Kombisensor RA-Gas GmbH", 0, 0, 0);
    sensors.add (sensor);

    return sensors;
  }

}
}
