#ifndef __XMZAPP_H
#define __XMZAPP_H

#include <gtk/gtk.h>


#define XMZ_APP_TYPE (xmz_app_get_type ())
#define XMZ_APP(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), XMZ_APP_TYPE, XMZApp))


typedef struct _XMZApp        XMZApp;
typedef struct _XMZAppClass   XMZAppClass;


GType           xmz_app_get_type    (void);
XMZApp          *xmz_app_new        (void);


#endif  /* __XMZAPP_H */
