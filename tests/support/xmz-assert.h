

#ifndef __XMZ_ASSERT_H__
#define __XMZ_ASSERT_H__

#include <glib.h>

#define xmz_test_assert_assert_no_error(error) g_assert_no_error(error)
#define xmz_test_assert_assert_streq(a, b) g_assert_cmpstr((a), ==, (b))
#define xmz_test_assert_assert_inteq(a, b) g_assert_cmpint((a), ==, (b))
#define xmz_test_assert_assert_booleq(a, b) g_assert_cmpuint((a), ==, (guint)(b))
#define xmz_test_assert_assert_uinteq(a, b) g_assert_cmpuint((a), ==, (b))
#define xmz_test_assert_assert_floateq(a, b) g_assert_cmpfloat((a), ==, (b))
#define xmz_test_assert_assert_datetime(a, b) \
  g_assert_cmpstr ( g_date_time_format ((a), "%F %T %z"), \
                    ==, \
                    g_date_time_format ((b), "%F %T %z") \
)

#endif /* __XMZ_ASSERT_H__ */
