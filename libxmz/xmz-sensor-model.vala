namespace XMZ {

public enum SensorModelColumns {
  ID,
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
      case ID:
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
  private GenericArray<XMZ.Sensor> d_data;


  public SensorModel (owned GenericArray<Sensor>? data = null) {
    if (data == null) {
      this.d_data = new GenericArray<Sensor> ();
    } else {
      this.d_data = (owned) data;
    }
  }

  construct {
  }

  public Type get_column_type (int index) {
    return ((SensorModelColumns)index).type ();
  }

  public void add (int id, string name , int adc_value, int adc_at_nullgas, int adc_at_messgas) {
    d_data.add (new XMZ.Sensor (id, name, adc_value, adc_at_nullgas, adc_at_messgas));
  }

  public Gtk.TreeModelFlags get_flags () {
    return Gtk.TreeModelFlags.LIST_ONLY |
        Gtk.TreeModelFlags.ITERS_PERSIST;
  }

  public bool get_iter (out Gtk.TreeIter iter, Gtk.TreePath path) {
    if (path.get_depth () != 1 || d_data.length == 0) {
      return invalid_iter (out iter);
    }

    iter = Gtk.TreeIter ();
    iter.user_data = path.get_indices ()[0].to_pointer ();
    iter.stamp = this.d_stamp;
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
    Sensor sensor = d_data.get ((uint) (ulong) iter.user_data);
    val.init(get_column_type (column));

    switch (column) {
      case SensorModelColumns.ID:
        val.set_int (sensor.id);
        break;
      case SensorModelColumns.NAME:
        val.set_string (sensor.name);
        break;
      case SensorModelColumns.ADC_VALUE:
        val.set_int (sensor.adc_value);
        break;
      case SensorModelColumns.ADC_AT_NULLGAS:
        val.set_int (sensor.adc_at_nullgas);
        break;
      case SensorModelColumns.ADC_AT_MESSGAS:
        val.set_int (sensor.adc_at_messgas);
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

  private bool invalid_iter (out Gtk.TreeIter iter) {
    iter = Gtk.TreeIter ();
    iter.stamp = -1;
    return false;
  }

}
}
