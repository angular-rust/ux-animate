# Shapes and fills
last modified July 13, 2020

In this part of the Cairo tutorial, we will create some basic and more advanced shapes. We will fill them with solid colours, patterns and gradients. Gradients will be covered in a separate chapter.

## Basic shapes
The Cairo API has some basic functions to create simple shapes.

```cpp
static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgb(cr, 0.6, 0.6, 0.6);
  cairo_set_line_width(cr, 1);

  cairo_rectangle(cr, 20, 20, 120, 80);
  cairo_rectangle(cr, 180, 20, 80, 80);
  cairo_stroke_preserve(cr);
  cairo_fill(cr);

  cairo_arc(cr, 330, 60, 40, 0, 2*M_PI);
  cairo_stroke_preserve(cr);
  cairo_fill(cr);

  cairo_arc(cr, 90, 160, 40, M_PI/4, M_PI);
  cairo_close_path(cr);
  cairo_stroke_preserve(cr);
  cairo_fill(cr);

  cairo_translate(cr, 220, 180);
  cairo_scale(cr, 1, 0.7);
  cairo_arc(cr, 0, 0, 50, 0, 2*M_PI);
  cairo_stroke_preserve(cr);
  cairo_fill(cr);
}
```
In this example, we will create a rectangle, a square, a circle, an arc and an ellipse.

```cpp
cairo_rectangle(cr, 20, 20, 120, 80);
cairo_rectangle(cr, 180, 20, 80, 80);
```
The cairo_rectangle() is used to create both squares and rectangles. A square is just a specific type of a rectangle.

```cpp
cairo_arc(cr, 330, 60, 40, 0, 2*M_PI);
```
This line creates a circle.

```cpp
cairo_scale(cr, 1, 0.7);
cairo_arc(cr, 0, 0, 50, 0, 2*M_PI);
```
We use the cairo_scale() function call to create an ellipse.

(Basic shapes) Figure: Basic shapes

Other shapes can be created using a combination of basic primitives.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *);

int points[11][2] = { 
    { 0, 85 }, 
    { 75, 75 }, 
    { 100, 10 }, 
    { 125, 75 }, 
    { 200, 85 },
    { 150, 125 }, 
    { 160, 190 },
    { 100, 150 }, 
    { 40, 190 },
    { 50, 125 },
    { 0, 85 } 
};

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{
  do_drawing(cr);

  return FALSE;
}

static void do_drawing(cairo_t *cr)
{ 
  cairo_set_source_rgb(cr, 0.6, 0.6, 0.6);
  cairo_set_line_width(cr, 1);

  gint i;
  for (i = 0; i < 10; i++) {
      cairo_line_to(cr, points[i][0], points[i][1]);
  }

  cairo_close_path(cr);
  cairo_stroke_preserve(cr);
  cairo_fill(cr);

  cairo_move_to(cr, 240, 40);
  cairo_line_to(cr, 240, 160);
  cairo_line_to(cr, 350, 160);
  cairo_close_path(cr);

  cairo_stroke_preserve(cr);
  cairo_fill(cr);

  cairo_move_to(cr, 380, 40);
  cairo_line_to(cr, 380, 160);
  cairo_line_to(cr, 450, 160);
  cairo_curve_to(cr, 440, 155, 380, 145, 380, 40);

  cairo_stroke_preserve(cr);
  cairo_fill(cr);  
}

int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER(window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL);  
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 460, 240); 
  gtk_window_set_title(GTK_WINDOW(window), "Other shapes");

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In this example, we create a star object a triangle and a modified triangle. These objects are created using lines and one curve.

```cpp
gint i;
for (i = 0; i < 10; i++ ) {
    cairo_line_to(cr, points[i][0], points[i][1]);
}

cairo_close_path(cr);
```

The star is drawn by joining all the points that are in the points array. The star is finished by calling the cairo_close_path() function, which joins the last two points of a star.

```cpp
cairo_move_to(cr, 380, 40);
cairo_line_to(cr, 380, 160);
cairo_line_to(cr, 450, 160);
cairo_curve_to(cr, 440, 155, 380, 145, 380, 40);
```
The modified triangle is a simple combination of two lines and one curve.

(Other shapes) Figure: Other shapes

## Fills
Fills fill the interiors of shapes. Fills can be solid colours, patters or gradients.

### Solid colours
A colour is an object representing a combination of Red, Green, and Blue (RGB) intensity values. Cairo valid RGB values are in the range 0 to 1.

```cpp
static void do_drawing(cairo_t *cr)
{ 
  cairo_set_source_rgb(cr, 0.5, 0.5, 1);
  cairo_rectangle(cr, 20, 20, 100, 100);
  cairo_fill(cr);

  cairo_set_source_rgb(cr, 0.6, 0.6, 0.6);
  cairo_rectangle(cr, 150, 20, 100, 100);
  cairo_fill(cr);
 
  cairo_set_source_rgb(cr, 0, 0.3, 0);
  cairo_rectangle(cr, 20, 140, 100, 100);
  cairo_fill(cr);

  cairo_set_source_rgb(cr, 1, 0, 0.5);
  cairo_rectangle(cr, 150, 140, 100, 100);
  cairo_fill(cr);  
}
```
In the example we draw four coloured rectangles.

```cpp
cairo_set_source_rgb(cr, 0.5, 0.5, 1);
cairo_rectangle(cr, 20, 20, 100, 100);
cairo_fill(cr);
```
The cairo_set_source_rgb() function call sets the source to an opaque colour. The parameters are the Red, Green, and Blue intensity values. The source is used to fill the interior of a rectangle by calling the cairo_fill() function.

(Solid colours) Figure: Solid colours

### Patterns
Patterns are complex graphical objects that can fill the shapes.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *);

cairo_surface_t *surface1;
cairo_surface_t *surface2;
cairo_surface_t *surface3;
cairo_surface_t *surface4;

static void create_surfaces() {
  surface1 = cairo_image_surface_create_from_png("blueweb.png");
  surface2 = cairo_image_surface_create_from_png("maple.png");
  surface3 = cairo_image_surface_create_from_png("crack.png");
  surface4 = cairo_image_surface_create_from_png("chocolate.png");
}

static void destroy_surfaces() {
  cairo_surface_destroy(surface1);
  cairo_surface_destroy(surface2);
  cairo_surface_destroy(surface3);
  cairo_surface_destroy(surface4);
}

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{
  do_drawing(cr);
  
  return FALSE;
}

static void do_drawing(cairo_t *cr)
{
  cairo_pattern_t *pattern1;
  cairo_pattern_t *pattern2;
  cairo_pattern_t *pattern3;
  cairo_pattern_t *pattern4;

  pattern1 = cairo_pattern_create_for_surface(surface1);
  pattern2 = cairo_pattern_create_for_surface(surface2);
  pattern3 = cairo_pattern_create_for_surface(surface3);
  pattern4 = cairo_pattern_create_for_surface(surface4);

  cairo_set_source(cr, pattern1);
  cairo_pattern_set_extend(cairo_get_source(cr), CAIRO_EXTEND_REPEAT);
  cairo_rectangle(cr, 20, 20, 100, 100);
  cairo_fill(cr);

  cairo_set_source(cr, pattern2); 
  cairo_pattern_set_extend(cairo_get_source(cr), CAIRO_EXTEND_REPEAT); 
  cairo_rectangle(cr, 150, 20, 100, 100);
  cairo_fill(cr);

  cairo_set_source(cr, pattern3);
  cairo_pattern_set_extend(cairo_get_source(cr), CAIRO_EXTEND_REPEAT);
  cairo_rectangle(cr, 20, 140, 100, 100);
  cairo_fill(cr);

  cairo_set_source(cr, pattern4);
  cairo_pattern_set_extend(cairo_get_source(cr), CAIRO_EXTEND_REPEAT);
  cairo_rectangle(cr, 150, 140, 100, 100);
  cairo_fill(cr);

  cairo_pattern_destroy(pattern1);
  cairo_pattern_destroy(pattern2);
  cairo_pattern_destroy(pattern3);
  cairo_pattern_destroy(pattern4);      
}

int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  
  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER(window), darea);  

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL);  
  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  create_surfaces();

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 270, 260); 
  gtk_window_set_title(GTK_WINDOW(window), "Patterns");

  gtk_widget_show_all(window);

  gtk_main();

  destroy_surfaces();

  return 0;
}
```

In this example we draw four rectangles again. This time, we fill them with some patterns. We use four pattern images from the Gimp image manipulation program. We must retain the original size of those patterns, because we are going to tile them.

We create image surfaces outside the on_draw_event() function. It would not be efficient to read from harddisk each time, the window needs to be redrawn.

```cpp
pattern1 = cairo_pattern_create_for_surface(surface1);
```
We create a pattern from the surface by calling the cairo_pattern_create_for_surface() function.

```cpp
cairo_set_source(cr, pattern1);
cairo_pattern_set_extend(cairo_get_source(cr), CAIRO_EXTEND_REPEAT);
cairo_rectangle(cr, 20, 20, 100, 100);
cairo_fill(cr);
```
Here we draw our first rectangle. The cairo_set_source() tells the Cairo context to use a pattern as a source for drawing. The image patterns may not fit exactly the shape. We set the mode to CAIRO_EXTEND_REPEAT, which causes the pattern to be tiled by repeating. The cairo_rectangle() creates a rectangular path. Finally, cairo_fill() fills the path with the source.

This chapter covered Cairo shapes and fills.