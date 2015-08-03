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
xmz_app_window_init (XMZAppWindow *app)
{
}

static void
xmz_app_window_class_init (XMZAppWindowClass *class)
{
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
