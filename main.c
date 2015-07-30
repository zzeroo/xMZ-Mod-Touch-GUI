#include <gtk/gtk.h>

#include "xmz-mod-touch-app.h"

int
main (int arc, char *argv[])
{
    return g_application_run (G_APPLICATION (xmz-mod-touch_app_new ()), argc, argv);
}
