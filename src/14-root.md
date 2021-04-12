# Root window
last modified July 13, 2020

In this part of the Cairo graphics tutorial, we will work with the root window. The root window is the desktop window where we usually have icon shortcuts.

It is possible to manipulate with the root window. From the programmer's perspective, it is just a special kind of a window.

## Transparent window
Our first example will create a transparent window. We will see, what it beneath of the window object.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *);


static void tran_setup(GtkWidget *win)
{        
  GdkScreen *screen;
  GdkVisual *visual;
  
  gtk_widget_set_app_paintable(win, TRUE);
  screen = gdk_screen_get_default();
  visual = gdk_screen_get_rgba_visual(screen);
  
  if (visual != NULL && gdk_screen_is_composited(screen)) {
      gtk_widget_set_visual(win, visual);
  }
}

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);  

  return FALSE;
}

static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgba(cr, 0.2, 0.2, 0.2, 0.4);
  cairo_set_operator(cr, CAIRO_OPERATOR_SOURCE);
  cairo_paint(cr);
}

int main (int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  
  tran_setup(window);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 250); 
  gtk_window_set_title(GTK_WINDOW(window), "Transparent window");

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
To create a transparent window, we get the visual of the screen object and set it for our window. In the on_draw() method, we draw over the screen's visual object. This creates an illusion of partial transparency.

```cpp
gtk_widget_set_app_paintable(win, TRUE);
```
We must set the application to be painted on.

```cpp
screen = gdk_screen_get_default();
```
The gdk_screen_get_default() method returns the screen object.

```cpp
visual = gdk_screen_get_rgba_visual(screen);
```
From the screen window, we get its visual. The visual contains the low level display information.

```cpp
if (visual != NULL && gdk_screen_is_composited(screen)) {
    gtk_widget_set_visual(win, visual);
}
```
Not all displays support this operation. Therefore, we check if our screen supports composition and the returned visual is not None. We set the screen's visual to be the visual of our window.

```cpp
static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgba(cr, 0.2, 0.2, 0.2, 0.4);
  cairo_set_operator(cr, CAIRO_OPERATOR_SOURCE);
  cairo_paint(cr);
}
```
We use a partially transparent source to draw over the screen window. The CAIRO_OPERATOR_SOURCE creates a composition operation where we draw over the source. Which is the screen window. To get full transparency, we set the alpha value to 0 or use the CAIRO_OPERATOR_CLEAR operator.

(Transparent window) Figure: Transparent window

## Taking a screenshot
The root window is also essential in taking a screenshot.

```cpp
#include <cairo.h>
#include <gdk/gdk.h>

int main (int argc, char *argv[])
{
  gdk_init(&argc, &argv);
  
  GdkWindow *root_win = gdk_get_default_root_window();
  gint width = gdk_window_get_width(root_win);
  gint height = gdk_window_get_height(root_win);

  cairo_surface_t *surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32,
      width, height);
    
  GdkPixbuf *pb = gdk_pixbuf_get_from_window(root_win, 0, 0, width, height);

  cairo_t *cr = cairo_create(surface);        
  gdk_cairo_set_source_pixbuf(cr, pb, 0, 0);  
  cairo_paint(cr);  

  cairo_surface_write_to_png(surface, "image.png");
  
  cairo_destroy(cr);
  cairo_surface_destroy(surface);

  return 0;
}
```
The example captures a snapshot of the entire screen. In this example, we do not use the full GTK windowing system. We use Cairo and GDK libraries to do the job.

```cpp
gdk_init(&argc, &argv);
```
The gdk_init() initializes the GDK library and connects to the windowing system.

```cpp
GdkWindow *root_win = gdk_get_default_root_window();
```
We get the root window with the gdk_get_default_root_window() function call.

```cpp
gint width = gdk_window_get_width(root_win);
gint height = gdk_window_get_height(root_win);
```
We determine the width and the height of the root window.

```cpp
cairo_surface_t *surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32,
    width, height);
```
An empty image surface is created. It has the size of the root window.

```cpp
GdkPixbuf *pb = gdk_pixbuf_get_from_window(root_win, 0, 0, width, height);
```
We get a pixbuf from the root window using the gdk_pixbuf_get_from_window() function call. A pixbuf is an object that describes an image in memory.

```cpp
cairo_t *cr = cairo_create(surface);        
gdk_cairo_set_source_pixbuf(cr, pb, 0, 0);  
cairo_paint(cr);
```
In the above code lines, we create a Cairo drawing context on the image surface that we have created earlier. We place the pixbuf on the drawing context and paint it on the surface.

```cpp
cairo_surface_write_to_png(surface, "image.png");
```
The image surface is written to a PNG image using the write_to_png() method.

```cpp
cairo_destroy(cr);
cairo_surface_destroy(surface);
```
We clean up resources.

## Showing a message
In the third example, we will show a message on the desktop window.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>
#include <pango/pango.h>

static void do_drawing(cairo_t *);

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);  

  return FALSE;
}

static void do_drawing(cairo_t *cr)
{
   cairo_set_operator(cr, CAIRO_OPERATOR_CLEAR);
   cairo_paint(cr);
   cairo_set_operator(cr, CAIRO_OPERATOR_OVER);
}

static void setup(GtkWidget *win)
{        
  gtk_widget_set_app_paintable(win, TRUE);
  gtk_window_set_type_hint(GTK_WINDOW(win), GDK_WINDOW_TYPE_HINT_DOCK);
  gtk_window_set_keep_below(GTK_WINDOW(win), TRUE);
  
  GdkScreen *screen = gdk_screen_get_default();
  GdkVisual *visual = gdk_screen_get_rgba_visual(screen);
  
  if (visual != NULL && gdk_screen_is_composited(screen)) {
      gtk_widget_set_visual(win, visual);
  }  
}

int main (int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *lbl;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  
  setup(window);
  
  lbl = gtk_label_new("ZetCode, tutorials for programmers");
  
  PangoFontDescription *fd = pango_font_description_from_string("Serif 20");
  gtk_widget_modify_font(lbl, fd);  
  gtk_container_add(GTK_CONTAINER(window), lbl);  
  
  GdkColor color;
  gdk_color_parse("white", &color);
  gtk_widget_modify_fg(lbl, GTK_STATE_NORMAL, &color);
  
  g_signal_connect(G_OBJECT(window), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 250); 

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
The code displays a message label on the root window.

```cpp
static void do_drawing(cairo_t *cr)
{
   cairo_set_operator(cr, CAIRO_OPERATOR_CLEAR);
   cairo_paint(cr);
   cairo_set_operator(cr, CAIRO_OPERATOR_OVER);
}
```
We use the CAIRO_OPERATOR_CLEAR operator to clear the background of the window. Then we set the CAIRO_OPERATOR_OVER to let the label widget be drawn.

```cpp
gtk_widget_set_app_paintable(win, TRUE);
```
We will be manipulating the application window, so we make it paintable.

```cpp
gtk_window_set_type_hint(GTK_WINDOW(win), GDK_WINDOW_TYPE_HINT_DOCK);
```
Implementing this window hint removes window borders and decoration.

```cpp
gtk_window_set_keep_below(GTK_WINDOW(win), TRUE);
```
We keep the application always at the bottom, just over the root window.

```cpp
GdkScreen *screen = gdk_screen_get_default();
GdkVisual *visual = gdk_screen_get_rgba_visual(screen);

if (visual != NULL && gdk_screen_is_composited(screen)) {
    gtk_widget_set_visual(win, visual);
}
```
We set the visual of the screen to be the visual of our application.

```cpp
lbl = gtk_label_new("ZetCode, tutorials for programmers");
```
We create a message label.

```cpp
PangoFontDescription *fd = pango_font_description_from_string("Serif 20");
gtk_widget_modify_font(lbl, fd);
```
With the help of the Pango module, we select a specific font for the text.

```cpp
gtk_container_add(GTK_CONTAINER(window), lbl);
```
The label is put onto the window.

```cpp
GdkColor color;
gdk_color_parse("white", &color);
gtk_widget_modify_fg(lbl, GTK_STATE_NORMAL, &color);
```
We modify the text to be in white colour.

(Message on the root window) Figure: Message on the root window

In this chapter we have worked with the desktop window in Cairo.