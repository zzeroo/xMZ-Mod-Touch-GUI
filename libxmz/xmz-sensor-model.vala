namespace XMZ {

public enum SensorModelColumns {
  NAME,
  DESCRIPTION,
  ADC_VALUE,
  ADC_AT_NULLGAS,
  ADC_AT_MESSGAS,
  NUM;

  public Type type () {
    switch (this) {
      case NAME:
      case DESCRIPTION:
        return typeof (string);
      case ADC_VALUE:
      case ADC_AT_NULLGAS:
      case ADC_AT_MESSGAS:
        return typeof (int);
      default:
        break;
    }

    return Type.INVALID;
  }
}

public class SensorModel : Object, Gtk.TreeModel {

  private uint d_size;
  private int d_stamp;

  construct {
  }


  public new Sensor? @get (uint idx) {
    return null;
  }

  public Type get_column_type (int index) {
    return ((SensorModelColumns)index).type ();
  }

  public Gtk.TreeModelFlags get_flags () {
    return Gtk.TreeModelFlags.LIST_ONLY |
        Gtk.TreeModelFlags.ITERS_PERSIST;
  }

  public bool get_iter (out Gtk.TreeIter iter, Gtk.TreePath path) {
    iter = {};

    int [] indices = path.get_indices ();

    if (indices.length != 1) {
      return false;
    }

    uint index = (uint) indices[0];

    if (index >= d_size) {
      return false;
    }

    iter.user_data = (void *) (ulong) index;
    iter.stamp = d_stamp;

    return true;
  }

  public int get_n_columns () {
    return SensorModelColumns.NUM;
  }

  public Gtk.TreePath? get_path (Gtk.TreeIter iter) {
    uint id = (uint) (ulong) iter.user_data;

    return_val_if_fail (iter.stamp == d_stamp, null);

    return new Gtk.TreePath.from_indices ((int) id);
  }

  public void get_value (Gtk.TreeIter iter, int column, out Value val) {
    val = {};

    return_if_fail (iter.stamp == d_stamp);

    uint idx = (uint) (ulong) iter.user_data;
    Sensor? sensor = this[idx];

    val.init (get_column_type (column));

    if (sensor == null) {
       return;
    }

    switch (column) {
      case SensorModelColumns.NAME:
        val.set_string (sensor.name);
        break;
      case SensorModelColumns.DESCRIPTION:
        val.set_string (sensor.description);
        break;
      case SensorModelColumns.ADC_VALUE:
        val.set_string (sensor.adc_value.to_string ());
        break;
      case SensorModelColumns.ADC_AT_NULLGAS:
        val.set_string (sensor.adc_at_nullgas.to_string ());
        break;
      case SensorModelColumns.ADC_AT_MESSGAS:
        val.set_string (sensor.adc_at_messgas.to_string ());
        break;
    }
  }

  public bool iter_children (out Gtk.TreeIter iter, Gtk.TreeIter? parent) {
    iter = {};

    if (parent == null) {
      iter.user_data = (void *) (ulong)0;
      iter.stamp = d_stamp;

      return true;
    } else {
      return_val_if_fail (parent.stamp == d_stamp, false);
      return false;
    }
  }

  public bool iter_has_child (Gtk.TreeIter iter) {
     return false;
  }

  public int iter_n_children (Gtk.TreeIter? iter) {
    if (iter == null) {
      return (int)d_size;
    } else {
      return_val_if_fail (iter.stamp == d_stamp, 0);
      return 0;
    }
  }

  public bool iter_next (ref Gtk.TreeIter iter) {
    return_val_if_fail (iter.stamp == d_stamp, false);

    uint index = (uint) (ulong) iter.user_data;
    ++index;

    if (index >= d_size) {
      return false;
    } else {
      iter.user_data = (void *) (ulong) index;
      return true;
    }
  }

  public bool iter_nth_child (out Gtk.TreeIter iter, Gtk.TreeIter? parent, int n) {
    iter = {};

    if (parent != null || (uint)n >= d_size) {
      return false;
    }

    iter.user_data = (void *) (ulong) n;
    iter.stamp = d_stamp;

    return true;
  }

  public bool iter_parent (out Gtk.TreeIter parent, Gtk.TreeIter iter) {
    parent = {};

    return_val_if_fail (iter.stamp == d_stamp, false);

    return false;
  }

}
}
