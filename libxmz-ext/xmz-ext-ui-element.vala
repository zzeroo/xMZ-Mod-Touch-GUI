

namespace XMZExt {

/**
 * xmx UIElement interface.
 *
 */
public interface UIElement : Object {
  /**
   * The main xmz application interface.
   *
   * This property is a "construct"
   * property and will be automatically set when an instance of ui element
   * object is created.
   */
  public abstract XMZExt.Application? application { owned get; construct set; }

  /**
   * A unique id for the ui element.
   *
   * Ids in xmz ar normally of the form /com/gaswarnanlagen/xmz/...
   */
  public abstract string id { owned get; }

  /**
   * The display name of the ui element.
   *
   * This should result in a string which can
   * be displayed in the xmz UI to identify the element.
   */
  public abstract string display_name { owned get; }

  /**
   * The description of the ui element.
   *
   * This should result in a string which can
   * be displayed in the xmz UI to describe the element.
   */
  public abstract string description { owned get; }

  /**
   * The UI element icon.
   *
   * If provided, the icon will be used in navigation toolbars
   * so that the users can switch to the UI element.
   */
  public virtual string? icon {
    owned get { return null; }
  }

  /**
   * The UI element widget.
   *
   * This widget will be embedded in the xmz UI when
   * the element is activated.
   */
  public virtual Gtk.Widget? widget {
    owned get { return null; }
  }

  /**
   * Check whether the UI element is available in the current application state.
   *
   * This method is used by xmz to verify whether or not a particular UI
   * element is available given the current state of the application. If the
   * element is not available, it will not be shown.
   *
   */
  public virtual bool available {
    get { return true; }
  }

  /**
   * Check whether the UI element is enabled in the current application state.
   *
   * This method is used by xmz to verify whether or not a particular UI
   * element is enabled (sensitive) given the current state of the application.
   *
   */
  public virtual bool enabled {
    get { return true; }
  }

  /**
   * Negotiate the order with another UIElement.
   *
   * This method is used to determine the order in which elements need to
   * appear in the UI.
   *
   * @return -1 if the element should appear before @other, 1 if the
   * element should appear after @other and 0 if the other is
   * unimportant.
   *
   */
  public virtual int negotiate_order (UIElement other) {
    return 0;
  }

  /**
   * Activate the UIElement.
   *
   * This signal is emitted when the UIElement has been activated.
   * Implementation can override the default handler to do any necessary
   * setup when the UI element is activated.
   */
  public virtual signal void activate() {
  }

}
}

// ex: ts=4 noet
