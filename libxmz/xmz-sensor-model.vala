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
  private GenericArray<Sensor> data;
  private int stamp = 0;


  public SensorModel (owned GenericArray<Sensor>? data = null) {
    if (data == null) {
      this.data = new GenericArray<Sensor> ();
    } else {
      this.data = (owned) data;
    }
  }

  public void add (int id, string name , int adc_value, int adc_at_nullgas, int adc_at_messgas) {
    data.add (new Sensor (id, name, adc_value, adc_at_nullgas, adc_at_messgas));
    stamp++;
  }

  public Type get_column_type (int index) {
    return ((SensorModelColumns)index).type ();
  }

  public Gtk.TreeModelFlags get_flags () {
    /* return Gtk.TreeModelFlags.LIST_ONLY | */
    /*     Gtk.TreeModelFlags.ITERS_PERSIST; */
    return 0;
  }

  public void get_value (Gtk.TreeIter iter, int column, out Value val) {
    assert (iter.stamp == stamp);

    val.init(get_column_type (column));
    Sensor sensor = data.get ((int) iter.user_data);
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
      default:
        break;
    }
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

  public int iter_n_children (Gtk.TreeIter? iter) {
    assert (iter == null || iter.stamp == stamp);
    return (iter == null) ? data.length : 0;
  }

  public bool iter_next (ref Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);

    int pos = ( (int) iter.user_data ) + 1;
    if (pos >= data.length) {
      return false;
    }
    iter.user_data = pos.to_pointer ();
    return true;
  }





  public bool iter_children (out Gtk.TreeIter iter, Gtk.TreeIter? parent) {
    assert (parent == null || parent.stamp == stamp);
    // Only used for trees
    return invalid_iter (out iter);
  }

  public bool iter_has_child (Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);
    // Only used for trees
    return false;
  }

  // FIXME: Refactor in iter_next () style
  public bool iter_previous (ref Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);

    int pos = (int) iter.user_data;
    if (pos >= 0) {
      return false;
    }
    iter.user_data = (--pos).to_pointer ();
    return true;
  }

  public bool iter_nth_child (out Gtk.TreeIter iter, Gtk.TreeIter? parent, int n) {
    assert (parent == null || parent.stamp == stamp);

    if (parent == null && n < data.length) {
      iter = Gtk.TreeIter ();
      iter.stamp = stamp;
      iter.user_data = n.to_pointer ();
      return true;
    }
    // Only used for trees
    return invalid_iter (out iter);
  }

  public bool iter_parent (out Gtk.TreeIter parent, Gtk.TreeIter child) {
    assert (child.stamp == stamp);
    // Only used for trees
    return invalid_iter (out parent);
  }

  private bool invalid_iter (out Gtk.TreeIter iter) {
    iter = Gtk.TreeIter ();
    iter.stamp = -1;
    return false;
  }

}
}
