# Clipping and masking
last modified July 13, 2020

In this part of the Cairo tutorial, we will talk about clipping and masking.

## Clipping
Clipping is restricting of drawing to a certain area. This is done for efficiency reasons and to create interesting effects.

In the following example we will be clipping an image.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>
#include <math.h>

static void do_drawing(cairo_t *, GtkWidget *);

struct {
  cairo_surface_t *image;
} glob;

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr, widget);

  return FALSE;
}

static void do_drawing(cairo_t *cr, GtkWidget *widget)
{
  static gint pos_x = 128;
  static gint pos_y = 128;
  static gint radius = 40;  
  static gint delta[] = { 3, 3 };

  GtkWidget *win = gtk_widget_get_toplevel(widget);
  
  gint width, height;
  gtk_window_get_size(GTK_WINDOW(win), &width, &height);

  if (pos_x < 0 + radius) {
      delta[0] = rand() % 4 + 5;
  } else if (pos_x > width - radius) {
      delta[0] = -(rand() % 4 + 5);
  }

  if (pos_y < 0 + radius) {
      delta[1] = rand() % 4 + 5;
  } else if (pos_y > height - radius) {
      delta[1] = -(rand() % 4 + 5);
  }

  pos_x += delta[0];
  pos_y += delta[1];

  cairo_set_source_surface(cr, glob.image, 1, 1);
  cairo_arc(cr, pos_x, pos_y, radius, 0, 2*M_PI);
  cairo_clip(cr);
  cairo_paint(cr);      
}

static gboolean time_handler(GtkWidget *widget)
{  
  gtk_widget_queue_draw(widget);
  return TRUE;
}

int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;  
  gint width, height;  

  glob.image = cairo_image_surface_create_from_png("turnacastle.png");
  width = cairo_image_surface_get_width(glob.image);
  height = cairo_image_surface_get_height(glob.image); 

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL);  
  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), width+2, height+2); 
  gtk_window_set_title(GTK_WINDOW(window), "Clip image");

  gtk_widget_show_all(window);
  g_timeout_add(100, (GSourceFunc) time_handler, (gpointer) window);

  gtk_main();

  cairo_surface_destroy(glob.image);

  return 0;
}
```
In this example, we will clip an image. A circle is moving on the screen and showing a part of the underlying image. This is as if we looked through a hole.

```cpp
if (pos_x < 0 + radius) {
    delta[0] = rand() % 4 + 5;
} else if (pos_x > width - radius) {
    delta[0] = -(rand() % 4 + 5);
}
```
If the circle hits the left or the right side of the window, the direction of the circle movement changes randomly. Same applies for the top and bottom sides.

```cpp
cairo_set_source_surface(cr, glob.image, 1, 1);
cairo_arc(cr, pos_x, pos_y, radius, 0, 2*M_PI);
```
Here we draw the image and a circle. Notice that we do not draw onto the window at the moment, but only in memory.

```cpp
cairo_clip(cr);
```
The cairo_clip() sets a clipping region. The clipping region is the current path used. The current path was created by the cairo_arc() function call.

```cpp
cairo_paint(cr);
```
The cairo_paint() paints the current source everywhere within the current clip region.

```cpp
glob.image = cairo_image_surface_create_from_png("turnacastle.png");
```
An image surface is created from a PNG image using the cairo_image_surface_create_from_png() function.

(Clipping image) Figure: Clipping image

## Mask
Before the source is applied to the surface, it is filtered first. The mask is used as a filter. The mask determines where the source is applied and where not. Opaque parts of the mask allow to copy the source. Transparent parts do not let to copy the source to the surface.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>


static void do_drawing(cairo_t *);

struct {
  cairo_surface_t *surface;
} glob;

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);  

  return FALSE;
}

static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgb(cr, 0, 0, 0);  
  cairo_mask_surface(cr, glob.surface, 0, 0);
  cairo_fill(cr);      
}

static void create_surface()
{
  glob.surface = cairo_image_surface_create_from_png("omen.png");
}

static void destroy_surface()
{
  cairo_surface_destroy(glob.surface);
}


int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;  

  gtk_init(&argc, &argv);
  
  create_surface();

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER(window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL);  
  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 305, 100); 
  gtk_window_set_title(GTK_WINDOW(window), "Mask");
  
  gtk_widget_show_all(window);

  gtk_main();
  
  destroy_surface();

  return 0;
}
```
This small example clearly shows the basic idea behind the mask. The mask determines where to paint and where not to paint.

```cpp
static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgb(cr, 0, 0, 0);  
  cairo_mask_surface(cr, glob.surface, 0, 0);
  cairo_fill(cr);      
}
```
In the do_drawing() function, we use an image as a mask. And it is therefore displayed on the window.

(Applying a mask) Figure: Applying a mask

## Blind down effect
In this code example, we will blind down our image. This is similar to what we do with a roller-blind.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>


static void do_drawing(cairo_t *);

struct {
  cairo_surface_t *image;
  cairo_surface_t *surface;
  gboolean timer;
  gint img_width;
  gint img_height;
} glob;


static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);
  
  return FALSE;
}

static gboolean time_handler(GtkWidget *widget)
{
  if (!glob.timer) return FALSE;
  
  gtk_widget_queue_draw(widget);
  return TRUE;
}

static void do_drawing(cairo_t *cr)
{
  cairo_t *ic;    
  static gint h = 0;     
  
  ic = cairo_create(glob.surface);

  cairo_rectangle(ic, 0, 0, glob.img_width, h);
  cairo_fill(ic);

  h += 1;
  if ( h == glob.img_height) glob.timer = FALSE;
  
  cairo_set_source_surface(cr, glob.image, 10, 10);
  cairo_mask_surface(cr, glob.surface, 10, 10);
  
  cairo_destroy(ic);  
}

static void init_vars()
{  
  glob.timer = TRUE;
  glob.image = cairo_image_surface_create_from_png("beckov.png");  
  glob.img_width = cairo_image_surface_get_width(glob.image);
  glob.img_height = cairo_image_surface_get_height(glob.image);  
  glob.surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 
                     glob.img_width, glob.img_height);
}

static void cleanup()
{
  cairo_surface_destroy(glob.image);
  cairo_surface_destroy(glob.surface);   
}

int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;    

  gtk_init(&argc, &argv);
  
  init_vars();

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER(window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);
 
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 325, 250); 
  gtk_window_set_title(GTK_WINDOW(window), "Blind down");

  g_timeout_add(15, (GSourceFunc) time_handler, (gpointer) window);

  gtk_widget_show_all(window);

  gtk_main();
  
  cleanup();

  return 0;
}
```
The idea behind the blind down effect is simple. The image is h pixels high. We draw 0, 1, 2 ... lines of 1px height. Each cycle the portion of the image is 1px higher, until the whole image is visible.

```cpp
struct {
  cairo_surface_t *image;
  cairo_surface_t *surface;
  gboolean timer;
  gint img_width;
  gint img_height;
} glob;
```
In the glob structure, we will store two surfaces, a timer and the image width and height variables.

```cpp
static void init_vars()
{  
  glob.timer = TRUE;
  glob.image = cairo_image_surface_create_from_png("beckov.png");  
  glob.img_width = cairo_image_surface_get_width(glob.image);
  glob.img_height = cairo_image_surface_get_height(glob.image);  
  glob.surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 
                     glob.img_width, glob.img_height);
}
```
In the init_vars() function, we initiate the previously declared variables. The last line creates an empty image surface. It is going to be filled with lines of pixels from the image surface that we have created earlier.

```cpp
ic = cairo_create(glob.surface);
```
We create a cairo context from the empty image source.

```cpp
cairo_rectangle(ic, 0, 0, glob.img_width, h);
cairo_fill(ic);
```
We draw a rectangle into the initially empty image. The rectangle will be 1px higher each cycle. The image created this way will serve as a mask later.

```cpp
h += 1;
```
The height of the image to show is increased by one unit.

```cpp
if ( h == glob.img_height) glob.timer = FALSE;
```
We stop the timer function when we draw the whole image on the GTK window.

```cpp
cairo_set_source_surface(cr, glob.image, 10, 10);
cairo_mask_surface(cr, glob.surface, 10, 10);
```
The image of a castle is set as a source for painting. The cairo_mask_surface() paints the current source using the alpha channel of surface as a mask.

```cpp
static void cleanup()
{
  cairo_surface_destroy(glob.image);
  cairo_surface_destroy(glob.surface);   
}
```
In the cleanup() function we destroy the created surfaces.

This chapter was about clipping and masking in Cairo.