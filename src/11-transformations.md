# Transformations
last modified July 13, 2020

In this part of the Cairo graphics programming tutorial, we will talk about transformations.

An affine transform is composed of zero or more linear transformations (rotation, scaling or shear) and translation (shift). Several linear transformations can be combined into a single matrix. A rotation is a transformation that moves a rigid body around a fixed point. A scaling is a transformation that enlarges or diminishes objects. The scale factor is the same in all directions.

A translation is a transformation that moves every point a constant distance in a specified direction. A shear is a transformation that moves an object perpendicular to a given axis, with greater value on one side of the axis than the other.

## Translation
The following example describes a simple translation.

```cpp
static void do_drawing(cairo_t *cr)
{ 
  cairo_set_source_rgb(cr, 0.2, 0.3, 0.8);
  cairo_rectangle(cr, 10, 10, 30, 30);
  cairo_fill(cr);

  cairo_translate(cr, 20, 20);
  cairo_set_source_rgb(cr, 0.8, 0.3, 0.2);
  cairo_rectangle(cr, 0, 0, 30, 30);
  cairo_fill(cr);
  
  cairo_translate(cr, 30, 30);
  cairo_set_source_rgb(cr, 0.8, 0.8, 0.2);
  cairo_rectangle(cr, 0, 0, 30, 30);
  cairo_fill(cr);
  
  cairo_translate(cr, 40, 40);
  cairo_set_source_rgb(cr, 0.3, 0.8, 0.8);
  cairo_rectangle(cr, 0, 0, 30, 30);
  cairo_fill(cr);    
}
```
The examle draws a rectangle. Then we do a translation and draw the same rectangle again.

```cpp
cairo_translate(cr, 20, 20);
```
The cairo_translate() function modifies the current transormation matrix by translating the user space origin. In our case we shift the origin by 20 units in both directions.

(Translation) Figure: Translation

## Shear
In the following example, we perform a shearing operation. A shearing is an object distortion along a particular axis. There is no shear function for this operation. We need to create our own transformation matrix. Note that each affine transformation can be performed by creating a transformation matrix.

```cpp
static void do_drawing(cairo_t *cr)
{  
  cairo_matrix_t matrix;

  cairo_set_source_rgb(cr, 0.6, 0.6, 0.6);
  cairo_rectangle(cr, 20, 30, 80, 50);
  cairo_fill(cr);
  cairo_matrix_init(&matrix,
      1.0, 0.5,
      0.0, 1.0,
      0.0, 0.0);

  cairo_transform(cr, &matrix);
  cairo_rectangle(cr, 130, 30, 80, 50);
  cairo_fill(cr);
}
```
In this code example, we perform a simple shearing operation.

```cpp
cairo_matrix_t matrix;
```
The cairo_matrix_t is a structure that holds an affine transformation.

```cpp
cairo_matrix_init(&matrix,
    1.0, 0.5,
    0.0, 1.0,
    0.0, 0.0);
```
This transformation shears y values by 0.5 of the x values.

```cpp
cairo_transform(cr, &matrix);
```
We perform the transformation with the transform() method.

(Shearing) Figure: Shearing

## Scaling
The next example demonstrates a scaling operation. Scaling is a transformation operation where the object is enlarged or shrunken.

```cpp
static void do_drawing(cairo_t *cr)
{        
  cairo_set_source_rgb(cr, 0.2, 0.3, 0.8);
  cairo_rectangle(cr, 10, 10, 90, 90);    
  cairo_fill(cr);
  
  cairo_scale(cr, 0.6, 0.6);
  cairo_set_source_rgb(cr, 0.8, 0.3, 0.2);
  cairo_rectangle(cr, 30, 30, 90, 90);    
  cairo_fill(cr);  

  cairo_scale(cr, 0.8, 0.8);
  cairo_set_source_rgb(cr, 0.8, 0.8, 0.2);
  cairo_rectangle(cr, 50, 50, 90, 90);    
  cairo_fill(cr);      
}
```
We draw three rectangles of 90x90px size. On two of them, we perform a scaling operation.

```cpp
cairo_scale(cr, 0.6, 0.6);
cairo_set_source_rgb(cr, 0.8, 0.3, 0.2);
cairo_rectangle(cr, 30, 30, 90, 90);    
cairo_fill(cr);
```
We uniformly scale a rectangle by a factor of 0.6.

```cpp
cairo_scale(cr, 0.8, 0.8);
cairo_set_source_rgb(cr, 0.8, 0.8, 0.2);
cairo_rectangle(cr, 50, 50, 90, 90);    
cairo_fill(cr); 
```
Here we perform another scaling operation by a factor of 0.8. If we look at the picture, we see that the third yellow rectangle is the smallest one. Even if we have used a smaller scaling factor. This is because transformation operations are additive. In fact, the third rectangle was scaled by a factor of 0.528 (0.6x0.8).

(Scaling) Figure: Scaling

## Isolating transformations
Transformation operations are additive. To isolate one operation from the other one, we can use the cairo_save() and cairo_restore() functions. The cairo_save() function makes a copy of the current state of the drawing context and saves it on an internal stack of saved states. The cairo_restore() function will re-establish the context to the saved state.

```cpp
static void do_drawing(cairo_t *cr)
{     
  cairo_set_source_rgb(cr, 0.2, 0.3, 0.8);
  cairo_rectangle(cr, 10, 10, 90, 90);    
  cairo_fill(cr);
  
  cairo_save(cr);
  cairo_scale(cr, 0.6, 0.6);
  cairo_set_source_rgb(cr, 0.8, 0.3, 0.2);
  cairo_rectangle(cr, 30, 30, 90, 90);    
  cairo_fill(cr);
  cairo_restore(cr);

  cairo_save(cr);
  cairo_scale(cr, 0.8, 0.8);
  cairo_set_source_rgb(cr, 0.8, 0.8, 0.2);
  cairo_rectangle(cr, 50, 50, 90, 90);    
  cairo_fill(cr);        
  cairo_restore(cr);
}
```
In the example we scale two rectangles. This time we isolate the scaling operations from each other.

```cpp
cairo_save(cr);
cairo_scale(cr, 0.6, 0.6);
cairo_set_source_rgb(cr, 0.8, 0.3, 0.2);
cairo_rectangle(cr, 30, 30, 90, 90);    
cairo_fill(cr);
cairo_restore(cr);
```
We isolate the scaling operation by putting the cairo_scale() function between the cairo_save() and cairo_restore() functions.

(Isolating transformations) Figure: Isolating transformations

Now the third yellow rectangle is bigger than the second red one.

## Donut
In the following example we create an complex shape by rotating a bunch of ellipses.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>
#include <math.h>


static void do_drawing(cairo_t *, GtkWidget *widget);

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{        
  do_drawing(cr, widget);

  return FALSE;
}

static void do_drawing(cairo_t *cr, GtkWidget *widget)
{         
  GtkWidget *win = gtk_widget_get_toplevel(widget);
  
  gint width, height;
  gtk_window_get_size(GTK_WINDOW(win), &width, &height);

  cairo_set_line_width(cr, 0.5);
  cairo_translate(cr, width/2, height/2);
  cairo_arc(cr, 0, 0, 120, 0, 2 * M_PI);
  cairo_stroke(cr);

  gint i;
  for (i = 0; i < 36; i++) {
      cairo_save(cr);
      cairo_rotate(cr, i*M_PI/36);
      cairo_scale(cr, 0.3, 1);
      cairo_arc(cr, 0, 0, 120, 0, 2 * M_PI);
      cairo_restore(cr);
      cairo_stroke(cr);      
  }    
}


int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL);
  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 250); 
  gtk_window_set_title(GTK_WINDOW(window), "Donut");
  
  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
We will do rotation and scaling operations. We will also save and restore Cairo contexts.

```cpp
cairo_translate(cr, width/2, height/2);
cairo_arc(cr, 0, 0, 120, 0, 2 * M_PI);
cairo_stroke(cr);
```
In the middle of the GTK+ window, we create a circle. This will be a bounding circle for our ellipses.

```cpp
gint i;
for (i = 0; i < 36; i++) {
    cairo_save(cr);
    cairo_rotate(cr, i*M_PI/36);
    cairo_scale(cr, 0.3, 1);
    cairo_arc(cr, 0, 0, 120, 0, 2 * M_PI);
    cairo_restore(cr);
    cairo_stroke(cr);      
}
```
We create 36 ellipses along the path of our bounding circle. We insulate each rotate and scale operation from one another with the cairo_save() and cairo_restore() methods.

## Star
The next example shows a rotating and scaling star.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *, GtkWidget *widget);

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
  do_drawing(cr, widget);

  return FALSE;
}

static void do_drawing(cairo_t *cr, GtkWidget *widget)
{
  static gdouble angle = 0;
  static gdouble scale = 1;
  static gdouble delta = 0.01;

  GtkWidget *win = gtk_widget_get_toplevel(widget);
  
  gint width, height;
  gtk_window_get_size(GTK_WINDOW(win), &width, &height);

  cairo_set_source_rgb(cr, 0, 0.44, 0.7);
  cairo_set_line_width(cr, 1);

  cairo_translate(cr, width/2, height/2 );
  cairo_rotate(cr, angle);
  cairo_scale(cr, scale, scale);

  gint i;

  for ( i = 0; i < 10; i++ ) {
      cairo_line_to(cr, points[i][0], points[i][1]);
  }

  cairo_close_path(cr);
  cairo_fill(cr);
  cairo_stroke(cr);

  if ( scale < 0.01 ) {
      delta = -delta;
  } else if (scale > 0.99) {
      delta = -delta;
  }

  scale += delta;
  angle += 0.01;    
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

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  
  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);  

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);
 
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 400, 300); 
  gtk_window_set_title(GTK_WINDOW(window), "Star");

  g_timeout_add(10, (GSourceFunc) time_handler, (gpointer) window);  

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
In this example, we create a star object. We will translate it, rotate it, and scale it.

```cpp
int points[11][2] = { 
    { 0, 85 }, 
    { 75, 75 }, 
    { 100, 10 }, 
...
```
The star object will be constructed from these points.

```cpp
static gdouble angle = 0;
static gdouble scale = 1;
static gdouble delta = 0.01;
```
We initialize three important variables. The angle is used in the rotation, the scale in scaling the star object. The delta variable controls when the star is growing and when it is shrinking.

```cpp
cairo_translate(cr, width/2, height/2);
cairo_rotate(cr, angle);
cairo_scale(cr, scale, scale);
```
We shift the star into the middle of the window. Rotate it and scale it.

```cpp
gint i;
for ( i = 0; i < 10; i++ ) {
    cairo_line_to(cr, points[i][0], points[i][1]);
}

cairo_close_path(cr);
cairo_fill(cr);
cairo_stroke(cr);
```
Here we draw the star object.

```cpp
if ( scale < 0.01 ) {
    delta = -delta;
} else if (scale > 0.99) {
    delta = -delta;
}
```
These lines control the growing or shrinking of the star object.

In this part of the Cairo graphics tutorial, we talked about transformations.