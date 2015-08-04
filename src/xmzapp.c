#include <gtk/gtk.h>

#include "xmzapp.h"
#include "xmzappwin.h"

struct _XMZApp
{
    GtkApplication parent;
};

struct _XMZAppClass
{
    GtkApplicationClass parent_class;
};

G_DEFINE_TYPE(XMZApp, xmz_app, GTK_TYPE_APPLICATION);

static void
xmz_app_init (XMZApp *app)
{
}

static void
xmz_app_activate (GApplication *app)
{
    GdkGeometry hints;

    hints.base_width = 1024;
        hints.base_height = 600;
        hints.min_width = 1024;
        hints.min_height = 600;
        hints.max_width = 1024;
        hints.max_height = 600;

    XMZAppWindow *win;
    win = xmz_app_window_new (XMZ_APP (app));
    gtk_window_set_geometry_hints (GTK_WINDOW (win),
                                    GTK_WIDGET (win),
                                    &hints,
                                    GDK_HINT_RESIZE_INC |
                                    GDK_HINT_MIN_SIZE |
                                    GDK_HINT_BASE_SIZE |
                                    GDK_HINT_MAX_SIZE);

    if (TRUE)
        gtk_window_maximize (GTK_WINDOW (win));

    gtk_window_present (GTK_WINDOW (win));
}

static void
xmz_app_open (GApplication *app,
            GFile           **files,
            gint            n_files,
            const gchar     *hint)
{
    GList *windows;
    XMZAppWindow *win;
    int i;

    windows = gtk_application_get_windows (GTK_APPLICATION (app));
    if (windows)
        win = XMZ_APP_WINDOW (windows->data);
    else
        win = xmz_app_window_new (XMZ_APP (app));

    for (i = 0; i < n_files; i++)
        xmz_app_window_open (win, files[i]);

    gtk_window_present (GTK_WINDOW (win));
}

static void
xmz_app_class_init (XMZAppClass *class)
{
    G_APPLICATION_CLASS (class)->activate = xmz_app_activate;
    G_APPLICATION_CLASS (class)->open = xmz_app_open;
}

XMZApp *
xmz_app_new (void)
{
    return g_object_new (XMZ_APP_TYPE,
                        "application-id", "com.gaswarnanlagen.xmz-mod-touch",
                        "flags", G_APPLICATION_HANDLES_OPEN,
                        NULL);
}






