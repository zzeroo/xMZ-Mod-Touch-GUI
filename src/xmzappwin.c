#include <gtk/gtk.h>

#include "xmzapp.h"
#include "xmzappwin.h"

struct _XMZAppWindow
{
    GtkApplicationWindow parent;
};

struct _XMZAppWindowClass
{
    GtkApplicationWindowClass parent_class;
};

G_DEFINE_TYPE(XMZAppWindow, xmz_app_window, GTK_TYPE_APPLICATION_WINDOW);

static void
xmz_app_window_init (XMZAppWindow *win)
{
    gtk_widget_init_template (GTK_WIDGET (win));
}

static void
xmz_app_window_class_init (XMZAppWindowClass *class)
{
    gtk_widget_class_set_template_from_resource (GTK_WIDGET_CLASS (class),
                                                "/com/gaswarnanlagen/xmz_mod_touch/window.ui");
}

XMZAppWindow *
xmz_app_window_new (XMZApp *app)
{
    return g_object_new (XMZ_APP_WINDOW_TYPE, "application", app, NULL);
}

void
xmz_app_window_open (XMZAppWindow *win,
                    GFile           *file)
{
}
