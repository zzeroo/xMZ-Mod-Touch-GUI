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
  private GenericArray<Sensor> data;

  private uint size;
  private int stamp;

  public signal void update (uint added);

  public SensorModel (owned GenericArray<Sensor>? data = null) {
    if (data == null) {
      this.data = new GenericArray<Sensor> ();
    } else {
      this.data = (owned) data;
    }
  }

  construct {
    Timeout.add (100, update_sensors);
  }

  public void add (string name, int adc_value) {
    data.add (new Sensor (name, adc_value));
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
    if (path.get_depth () != 1 || data.length == 0) {
      return invalid_iter (out iter);
    }
    iter = Gtk.TreeIter ();
    iter.user_data = path.get_indices ()[0].to_pointer ();
    iter.stamp = this.stamp;

    return true;
  }

  public int get_n_columns () {
    return SensorModelColumns.NUM;
  }

  public Gtk.TreePath? get_path (Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);

    Gtk.TreePath path = new Gtk.TreePath ();
    path.append_index ((int) iter.user_data);

    return path;
  }

  public void get_value (Gtk.TreeIter iter, int column, out Value val) {
    assert (iter.stamp == stamp);

    uint idx = (uint) (ulong) iter.user_data;
    Sensor? sensor = data.get (idx);

    val.init (get_column_type (column));

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
    Sensor? sensor = data.get (idx);

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
    assert (iter == null || iter.stamp == stamp);
    return (iter == null) ? data.length : 0;
  }

  public bool iter_next (ref Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);

    int pos = ((int) iter.user_data) + 1;
    if (pos >= data.length) {
      return false;
    }
    iter.user_data = pos.to_pointer ();

    return true;
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


  private bool invalid_iter (out Gtk.TreeIter iter) {
    iter = Gtk.TreeIter ();
    iter.stamp =-1;
    return false;
  }

  private bool update_sensors () {
    data.foreach ((sensor) => {
                  stdout.printf ("Sensor: %s (%d)\n", sensor.name, sensor.adc_value);
                  sensor.adc_value += 1;
                  });

    return true;
  }
}
}
// ex:set ts=4 noet
