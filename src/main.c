#include <gtk/gtk.h>

#include "xmzapp.h"

int
main (int argc, char *argv[])
{
    return g_application_run (G_APPLICATION (xmz_app_new ()), argc, argv);
}
