namespace XMZ {

public enum SensorModelColumns {
  NAME,
  ADC_VALUE,
  NUM;

  public Type type () {
    switch (this) {
      case NAME:
        return typeof (string);
      case ADC_VALUE:
        return typeof (int);
      default:
        break;
    }
    return Type.INVALID;
  }
}

public class SensorModel : Object, Gtk.TreeModel {
  private Thread<void*>? thread;
  private GenericArray<Sensor> sensors;

  private uint size;
  private int stamp;


  public SensorModel (owned GenericArray<Sensor>? sensor = null) {
    if (sensors == null) {
      this.sensors = new GenericArray<Sensor> ();
    } else {
      this.sensors = (owned) sensors;
    }
  }

  construct {
  }

  public void add (string name, int adc_value) {
    sensors.add (new Sensor (name, adc_value));
    stamp++;
  }

  public Type get_column_type (int index) {
    return ((SensorModelColumns) index).type ();
  }

  // TODO: Get more about TreeModelFlags and fix if necessary
  public Gtk.TreeModelFlags get_flags () {
    return 0;
  }

  public bool get_iter (out Gtk.TreeIter iter, Gtk.TreePath path) {
    iter = {};

    int[] indices = path.get_indices ();

    if (indices.length != 1) {
      return false;
    }

    uint index = (uint) indices[0];

    if (index >= size) {
      return false;
    }

    iter.user_data = (void *) (ulong) index;
    iter.stamp = stamp;

    return true;
  }

  public int get_n_columns () {
    return SensorModelColumns.NUM;
  }

  public Gtk.TreePath? get_path (Gtk.TreeIter iter) {
    uint id = (uint) (ulong) iter.user_data;

    return_val_if_fail (iter.stamp == stamp, null);

    return new Gtk.TreePath.from_indices ((int) id);
  }

  public void get_value (Gtk.TreeIter iter, int column, out Value val) {
    val = {};

    return_if_fail (iter.stamp == stamp);

    uint idx = (uint) (ulong) iter.user_data;

    Sensor? sensor = sensors.get (idx);

    val.init (get_column_type (column));

    if (sensor == null) {
      return;
    }

    switch (column) {
      case SensorModelColumns.NAME:
        val.set_string (sensor.name);
        break;
      case SensorModelColumns.ADC_VALUE:
        val.set_int (sensor.adc_value);
        break;
    }
  }

  public Sensor? sensor_from_iter (Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);

    uint idx = (uint) (ulong) iter.user_data;
    Sensor? sensor = sensors.get (idx);

    return sensor;
  }

  public bool iter_children (out Gtk.TreeIter iter, Gtk.TreeIter? parent) {
    iter = {};

    if (parent == null) {
      iter.user_data = (void *) (ulong) 0;
      iter.stamp = stamp;

      return true;
    } else {
      return_val_if_fail (parent.stamp == stamp, false);
      return false;
    }
  }

  public bool iter_has_child (Gtk.TreeIter iter) {
    return false;
  }

  public int iter_n_children (Gtk.TreeIter? iter) {
    if (iter == null) {
      return (int) size;
    } else {
      return_val_if_fail (iter.stamp == stamp, 0);
      return 0;
    }
  }

  public bool iter_next (ref Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);

    uint index = (uint) (ulong) iter.user_data;
    ++index;

    if (index >= size) {
      return false;
    } else {
      iter.user_data = (void *) (ulong) index;
      return true;
    }
  }

  public bool iter_nth_child (out Gtk.TreeIter iter, Gtk.TreeIter? parent, int n) {
    iter = {};

    if (parent != null || (uint) n >= size) {
      return false;
    }

    iter.user_data = (void *) (ulong) n;
    iter.stamp = stamp;

    return true;
  }

  public bool iter_parent (out Gtk.TreeIter parent, Gtk.TreeIter iter) {
    parent = {};

    return_val_if_fail (iter.stamp == stamp, false);

    return false;
  }

}
}
