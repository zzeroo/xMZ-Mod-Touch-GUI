namespace XMZ {

public class DatabaseBackend : Object {
  // Do this to pull in config.h before glib.h (for gettext)
  private const string version = XMZ.Config.VERSION;

  private Sqlite.Database database;
  private int error_code;
  private string error_msg;
  private const string prepared_query_string = "SELECT * FROM Sensors WHERE id = $UID;";

  private Sensor sensor;
  private GenericArray<Sensor> sensors;

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



}
}
