#ifndef __XMZAPPWIN_H
#define __XMZAPPWIN_H

#include <gtk/gtk.h>
#include "xmzapp.h"


#define XMZ_APP_WINDOW_TYPE (xmz_app_window_get_type ())
#define XMZ_APP_WINDOW(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), XMZ_APP_WINDOW_TYPE, XMZAppWindow))


typedef struct _XMZAppWindow        XMZAppWindow;
typedef struct _XMZAppWindowClass   XMZAppWindowClass;


GType               xmz_app_window_get_type   (void);
XMZAppWindow        *xmz_app_window_new       (XMZApp *app);
void                xmz_app_window_open       (XMZAppWindow *win,
                                              GFile         *file);


#endif /* __XMZAPPWIN_H */
