
namespace XMZ {

public class UIElements<T> : Object {

  private Gee.HashMap<string, XMZExt.UIElement> d_elements;
  private List<XMZExt.UIElement> d_available_elements;
  private XMZExt.UIElement d_current;
  private Gtk.Stack d_stack;

  public T? current {
	get {
	  if (d_current != null) {
		return (T)d_current;
	  } else {
		return null;
	  }
	}

	set {
	  if (value != null) {
		set_current_impl ((XMZExt.UIElement)value);
	  }
	}
  }

  public T[] available_elements {
	owned get {
	  var ret = new T[0];

	  foreach (var elem in d_available_elements) {
		ret += (T)elem;
	  }

	  return ret;
	}
  }

  public void update () {}

  public void set_first_enabled_current () {}

  public T? lookup (string id) {
	return (T)d_elements[id];
  }

  private bool is_available (XMZExt.UIElement element) {
	return d_available_elements.find (element) != null;
  }

  private void set_current_impl (XMZExt.UIElement element) {
	if (!element.available ||
		!element.enabled ||
		(d_current != null && d_current == element) ||
		!is_available (element)) {
	  return;
	}

	if (d_current == element) {
	   return;
	}

	d_current = element;

	if (d_current == element) {
	  d_stack.set_visible_child (element.widget);
	}

	notify_property ("current");
	element.activate();
  }


  public delegate bool ForeachUIElementFunc (XMZExt.UIElement element);

  public void foreach (ForeachUIElementFunc func) {
	var vals = d_available_elements.copy ();

	foreach (var val in vals) {
	  if (!func (val)) {
		break;
	  }
	}
  }
}
}


// ex:ts=4 noet
