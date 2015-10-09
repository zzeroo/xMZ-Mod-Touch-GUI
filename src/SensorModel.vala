// Sensor Model
// http://references.valadoc.org/#!api=gtk+-3.0/Gtk.TreeModel

public class SensorModel : Object, Gtk.TreeModel {
  private GenericArray<SensorNode> data;
  private int stamp = 0;

  public SensorModel (owned GenericArray<SensorNode> ? data = null) {
    if (data == null) {
      this.data = new GenericArray<SensorNode> ();
    } else {
      this.data = (owned) data;
    }
  }

  public void add (int id, string name) {
    data.add (new SensorNode (id, name));
    stamp++;
  }

  public Type get_column_type (int index) {
    switch (index) {
      case 0:
        return typeof (int);
      case 1:
        return typeof (string);
      default:
        return Type.INVALID;
    }
  }

  public Gtk.TreeModelFlags get_flags () {
    return 0;
  }

  public void get_value (Gtk.TreeIter iter, int column, out Value val) {
     assert (iter.stamp == stamp);

     SensorNode node = data.get ((int) iter.user_data);
     switch (column) {
      case 0:
        val = Value (typeof (int));
        val.set_int (node.id);
        break;
      case 1:
        val = Value (typeof (string));
        val.set_string (node.name);
        break;
      default:
        val = Value (Type.INVALID);
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
    // id, name
    return 2;
  }

  public Gtk.TreePath? get_path (Gtk.TreeIter iter) {
    assert (iter.stamp == stamp);

    Gtk.TreePath path = new Gtk.TreePath ();
    path.append_index ((int) iter.user_data);
    return path;
  }

  public int iter_n_children (Gtk.TreeIter? iter) {
    assert (iter == null || iter.stamp == stamp);
    return (iter == null)? data.length : 0;
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

  public bool iter_parent (out Gtk.TreeIter iter, Gtk.TreeIter child) {
     assert (child.stamp == stamp);
     // Only used for trees
     return invalid_iter (out iter);
  }

  private bool invalid_iter (out Gtk.TreeIter iter) {
    iter = Gtk.TreeIter ();
    iter.stamp = -1;
    return false;
  }
}
