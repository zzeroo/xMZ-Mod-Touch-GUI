

namespace XMZ {

public class SensorListView : Gtk.TreeView, Gtk.Buildable {

  public SensorListView (SensorModel model) {
    Object (model: model);
  }

  public Gtk.CellRenderer? find_cell_at_pos (Gtk.TreeViewColumn column,
                                             Gtk.TreePath path,
                                             int x,
                                             out int width) {
    Gtk.TreeIter iter;

    model.get_iter (out iter, path);
    column.cell_set_cell_data (model, iter, false, false);

    var cells = column.get_cells ();

    foreach (var cell in cells) {
      int start;
      int cellw;

      if (!column.cell_get_position (cell, out start, out cellw)) {
        continue;
      }

      if (x >= start && x <= start + cellw) {
        width = cellw;
        return cell;
      }
    }

    width = 0;
    return null;
  }
}
}
