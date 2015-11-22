/**
 * This class handles all dirs and paths
 *
 * Inspiration from shotwells AppDirs class
 * https://github.com/GNOME/shotwell/blob/da21604830386a7c1f2b8d1f75a9d98719e5e5d1/src/AppDirs.vala
 */
namespace XMZ {

public class AppDirs {

  private const string DEFAULT_DATA_DIR = "xmz";

  private static File exec_dir;
  private static File data_dir = null;
  private static File tmp_dir = null;
  private static File libexec_dir = null;

  public static File get_data_dir () {
    return (data_dir == null) ? File.new_for_path (Environment.get_user_data_dir ()).get_child (DEFAULT_DATA_DIR) : data_dir;
  }

  public static File get_data_subdir (string name, string? subname = null) {
    File subdir = get_data_dir ().get_child (name);
    if (subname != null) {
      subdir = subdir.get_child (subname);
    }

    try {
      if (!subdir.query_exists (null)) {
        subdir.make_directory_with_parents (null);
      }
    } catch (Error e) {
      stdout.printf (_("Error could not create data subdirectory."));
    }

    return subdir;
  }
}
}
