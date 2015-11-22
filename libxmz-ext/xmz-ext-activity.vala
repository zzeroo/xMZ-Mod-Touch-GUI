

namespace XMZExt {

/**
 * xmz Activity interface
 *
 * The Activity interface can be implemented to provide a main activity in
 * xmz. An example of such activities are the builtin SensorList and
 * SensorValues activities.
 */
public interface Activity : Object, UIElement {
  /**
   * Whether the activity is the default for the specified action.
   *
   * @param action the action.
   * @return %TRUE if the activity is the default activity for @action,
   * %FALSE otherwise.
   *
   */
	public virtual bool is_default_for (string action) {
		return false;
	}
}
}

// ex: ts=4 noet
