# Images in Cairo
last modified July 13, 2020

In this part of the Cairo graphics tutorial, we will talk about the images. We will show how to display an image on the GTK window. We will also create some effects with images.

## Displaying an image
In the first example, we will display an image.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

struct {
  cairo_surface_t *image;  
} glob;


static void do_drawing(cairo_t *);

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);

  return FALSE;
}

static void do_drawing(cairo_t *cr)
{
  cairo_set_source_surface(cr, glob.image, 10, 10);
  cairo_paint(cr);    
}


int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;
  
  glob.image = cairo_image_surface_create_from_png("stmichaelschurch.png");

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(window, "destroy",
      G_CALLBACK (gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 220); 
  gtk_window_set_title(GTK_WINDOW(window), "Image");

  gtk_widget_show_all(window);

  gtk_main();

  cairo_surface_destroy(glob.image);

  return 0;
}
```
The example displays an image.

```cpp
glob.image = cairo_image_surface_create_from_png("stmichaelschurch.png");
```
We create an image surface from a PNG image. For efficiency reasons, the function is called in the main function.

```cpp
cairo_set_source_surface(cr, glob.image, 10, 10);
```
We create a source for painting from the created image surface.

```cpp
cairo_paint(cr);   
```
We paint the source on the window.

```cpp
cairo_surface_destroy(glob.image);
```
In the end, the surface is destroyed.

## Watermark
It is common to draw information on images. The text written on an image is called a watermark. Watermarks are used to identify images. They could be copyright notices or image creation times.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *, GtkWidget *widget);

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
  cairo_set_source_surface(cr, glob.image, 10, 10);
  cairo_paint(cr);
}

static void load_image()
{        
  glob.image = cairo_image_surface_create_from_png("beckov.png"); 
}

static void draw_mark() 
{ 
  cairo_t *ic;
  ic = cairo_create(glob.image);
  cairo_set_font_size(ic, 11);
  
  cairo_set_source_rgb(ic, 0.9 , 0.9 , 0.9);
  cairo_move_to(ic, 20, 30);
  cairo_show_text(ic, " Beckov 2012 , (c) Jan Bodnar ");
  cairo_stroke(ic);   
}

int main (int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;
  
  load_image();
  draw_mark();

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 250); 
  gtk_window_set_title(GTK_WINDOW(window), "Watermark");

  gtk_widget_show_all(window);

  gtk_main();
  
  cairo_surface_destroy(glob.image);

  return 0;
}
```

We draw copyright information on an image.

```cpp
static void load_image()
{        
  glob.image = cairo_image_surface_create_from_png("beckov.png"); 
}
```
In the load_image() method, we create an image surface from a PNG image.

```cpp
static void draw_mark() 
{ 
  cairo_t *ic;
  ic = cairo_create(glob.image);
...
```
In the draw_mark() function, we draw the copyright message on the image. First we create a drawing context from the image surface.

```cpp
cairo_set_font_size(ic, 11);

cairo_set_source_rgb(ic, 0.9 , 0.9 , 0.9);
cairo_move_to(ic, 20, 30);
cairo_show_text(ic, " Beckov 2012 , (c) Jan Bodnar ");
cairo_stroke(ic);   
```
Then we draw a small text in white colour.

```cpp
static void do_drawing(cairo_t *cr, GtkWidget *widget)
{
  cairo_set_source_surface(cr, glob.image, 10, 10);
  cairo_paint(cr);
}
```
The image surface is drawn on the window.

## The spectrum effect
We call this a spectrum effect, because it resembles and old ZX Spectrum computer. When you were loading an image into this computer, it was gradually appearing on the screen. The next example is loosely based on this experience.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *);

struct {
  gboolean timer;
  cairo_surface_t *image;
  cairo_surface_t *surface;
  gint img_width;
  gint img_height;
} glob;


static void init_vars()
{
  glob.image = cairo_image_surface_create_from_png("beckov.png");
  
  glob.img_width = cairo_image_surface_get_width(glob.image);
  glob.img_height = cairo_image_surface_get_height(glob.image);  
  
  glob.surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 
      glob.img_width, glob.img_height);    
  glob.timer = TRUE;   
}

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);
  
  return FALSE;
}

static void do_drawing(cairo_t *cr)
{
  cairo_t *ic;

  static gint count = 0;

  ic = cairo_create(glob.surface);

  gint i, j;
  for (i = 0; i <= glob.img_height; i+=7) {
      for (j = 0 ; j < count; j++) {
          cairo_move_to(ic, 0, i+j);
          cairo_line_to(ic, glob.img_width, i+j);
      }
  }

  count++;
  if (count == 8) glob.timer = FALSE;

  cairo_set_source_surface(cr, glob.image, 10, 10);
  cairo_mask_surface(cr, glob.surface, 10, 10);
  cairo_stroke(ic);

  cairo_destroy(ic);  
}

static gboolean time_handler(GtkWidget *widget)
{
  if (!glob.timer) return FALSE;

  gtk_widget_queue_draw(widget);
  return TRUE;
}


int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;

  init_vars();

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  
  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 325, 250); 
  gtk_window_set_title(GTK_WINDOW(window), "Spectrum");

  g_timeout_add(400, (GSourceFunc) time_handler, (gpointer) window);

  gtk_widget_show_all(window);

  gtk_main();

  cairo_surface_destroy(glob.image);
  cairo_surface_destroy(glob.surface);  

  return 0;
}
```
We divide the image into n parts consisting of 8 lines. Each cycle each part of the image will get bigger by one pixel. The created image will serve as a mask for displaying the image of the castle.

```cpp
struct {
  gboolean timer;
  cairo_surface_t *image;
  cairo_surface_t *surface;
  gint img_width;
  gint img_height;
} glob;
```
The glob structure stores variables that are used within more functions.

```cpp
static void init_vars()
{
  glob.image = cairo_image_surface_create_from_png("beckov.png");
  
  glob.img_width = cairo_image_surface_get_width(glob.image);
  glob.img_height = cairo_image_surface_get_height(glob.image);  
  
  glob.surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 
      glob.img_width, glob.img_height);    
  glob.timer = TRUE;   
}
```
In the init_vars() function, we initiate the aforementioned variables.

```cpp
gint i, j;
for (i = 0; i <= glob.img_height; i+=7) {
    for (j = 0 ; j < count; j++) {
        cairo_move_to(ic, 0, i+j);
        cairo_line_to(ic, glob.img_width, i+j);
    }
}
```
We gradully draw lines into each of the n parts.

```cpp
count++;
if (count == 8) glob.timer = FALSE;
```
After 8 steps, the animation finishes.

```cpp
cairo_set_source_surface(cr, glob.image, 10, 10);
cairo_mask_surface(cr, glob.surface, 10, 10);
cairo_stroke(ic);
```
Using the mask operation, we draw the portions of the image on the window.

This chapter covered images in Cairo.