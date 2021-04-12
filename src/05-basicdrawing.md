# Basic drawing in Cairo
last modified July 13, 2020

In this part of the Cairo graphics tutorial, we will draw some basic primitives. We will draw simple lines, use fill and stroke operations, we will talk about dashes, line caps and line joins.

## Lines
Lines are very basic vector objects. To draw a line, we use two function calls. The starting point is specified with the cairo_move_to() call. The ending point of a line is specified with the cairo_line_to() call.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *);

struct {
  int count;
  double coordx[100];
  double coordy[100];
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
  cairo_set_line_width(cr, 0.5);

  int i, j;
  for (i = 0; i <= glob.count - 1; i++ ) {
      for (j = 0; j <= glob.count - 1; j++ ) {
          cairo_move_to(cr, glob.coordx[i], glob.coordy[i]);
          cairo_line_to(cr, glob.coordx[j], glob.coordy[j]);
      }
  }

  glob.count = 0;
  cairo_stroke(cr);    
}

static gboolean clicked(GtkWidget *widget, GdkEventButton *event,
    gpointer user_data)
{
    if (event->button == 1) {
        glob.coordx[glob.count] = event->x;
        glob.coordy[glob.count++] = event->y;
    }

    if (event->button == 3) {
        gtk_widget_queue_draw(widget);
    }

    return TRUE;
}


int main(int argc, char *argv[])
{
  GtkWidget *window;
  GtkWidget *darea;
  
  glob.count = 0;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER(window), darea);
 
  gtk_widget_add_events(window, GDK_BUTTON_PRESS_MASK);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);  
    
  g_signal_connect(window, "button-press-event", 
      G_CALLBACK(clicked), NULL);
 
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 400, 300); 
  gtk_window_set_title(GTK_WINDOW(window), "Lines");

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
In our example, we click randomly on a window with a left mouse button. Each click is stored in an array. When we right click on the window, all points are connected with every point in the array. This way, we can create some interesting objects. Right clicking on the drawn object clears the window, and we can click another object.

```cpp
cairo_set_source_rgb(cr, 0, 0, 0);
cairo_set_line_width (cr, 0.5);
The lines will be drawn in black ink and will be 0.5 points wide.

int i, j;
for (i = 0; i <= glob.count - 1; i++ ) {
    for (j = 0; j <= glob.count - 1; j++ ) {
        cairo_move_to(cr, glob.coordx[i], glob.coordy[i]);
        cairo_line_to(cr, glob.coordx[j], glob.coordy[j]);
    }
}
```
We connect every point from the array to every other point.

```cpp
cairo_stroke(cr);
The cairo_stroke() call draws the lines.

g_signal_connect(window, "button-press-event", 
    G_CALLBACK(clicked), NULL);
```
We connect the button-press-event to the clicked callback.

```cpp
if (event->button == 1) {
    glob.coordx[glob.count] = event->x;
    glob.coordy[glob.count++] = event->y;
}
```
Inside the clicked callback, we determine if we there was a left or right click. If we click with a left mouse button, we store the x, y coordinates into the arrays.

```cpp
if (event->button == 3) {
    gtk_widget_queue_draw(widget);
}
```
By right clicking, we redraw the window.

(Lines) Figure: Lines

## Fill and stroke
The stroke operation draws the outlines of shapes and the fill operation fills the insides of shapes.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>
#include <math.h>

static void do_drawing(cairo_t *, GtkWidget *);

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{  
  do_drawing(cr, widget);  

  return FALSE;
}

static void do_drawing(cairo_t *cr, GtkWidget *widget)
{
  GtkWidget *win = gtk_widget_get_toplevel(widget);
  
  int width, height;
  gtk_window_get_size(GTK_WINDOW(win), &width, &height);
  
  cairo_set_line_width(cr, 9);  
  cairo_set_source_rgb(cr, 0.69, 0.19, 0);
  
  cairo_translate(cr, width/2, height/2);
  cairo_arc(cr, 0, 0, 50, 0, 2 * M_PI);
  cairo_stroke_preserve(cr);

  cairo_set_source_rgb(cr, 0.3, 0.4, 0.6); 
  cairo_fill(cr);      
}


int main (int argc, char *argv[])
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

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200); 
  gtk_window_set_title(GTK_WINDOW(window), "Fill & stroke");

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
In our example, we will draw a circle and fill it with a solid color.

```cpp
#include <math.h>
```
This header file is needed for the M_PI constant.

```cpp
GtkWidget *win = gtk_widget_get_toplevel(widget);

int width, height;
gtk_window_get_size(GTK_WINDOW(win), &width, &height);
```
Here we get the width and height of the window. We will need these values, when we draw the circle. The circle will be resized, when we resize the window.

```cpp
cairo_set_line_width(cr, 9);  
cairo_set_source_rgb(cr, 0.69, 0.19, 0);
```
We set a line width with the set_line_width() method. We set the source to some dark red colour using the set_source_rgb() method.

```cpp
cairo_translate(cr, width/2, height/2);
cairo_arc(cr, 0, 0, 50, 0, 2 * M_PI);
cairo_stroke_preserve(cr);
```
With the cairo_translate() method, we move the drawing origin to the center of the window. We want our circle to be centered. The arc() method adds a new circular path to the cairo drawing context. Finally, the stroke_preserve() method draws the outline of the circle. Unlike the stroke() method, it also preserves the shape for later drawing.

```cpp
cairo_set_source_rgb(cr, 0.3, 0.4, 0.6); 
cairo_fill(cr);
```
Here we fill the cirle with blue color.

(Fill and stroke) Figure: Fill and stroke

## Pen dashes
Each line can be drawn with a different pen dash. It defines the style of the line. The dash is used by the cairo_stroke() function call. The dash pattern is specified by the cairo_set_dash() function. The pattern is set by the dash array, which is an array of positive floating point values. They set the on and off parts of the dash pattern. We also specify the length of the array and the offset value. If the length is 0, the dashing is disabled. If it is 1, a symmetric pattern is asumed with alternating on and off portions of the size specified by the single value in dashes.

```cpp
static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgba(cr, 0, 0, 0, 1);

  static const double dashed1[] = {4.0, 21.0, 2.0};
  static int len1  = sizeof(dashed1) / sizeof(dashed1[0]);

  static const double dashed2[] = {14.0, 6.0};
  static int len2  = sizeof(dashed2) / sizeof(dashed2[0]);

  static const double dashed3[] = {1.0};

  cairo_set_line_width(cr, 1.5);

  cairo_set_dash(cr, dashed1, len1, 0);

  cairo_move_to(cr, 40, 30);  
  cairo_line_to(cr, 200, 30);
  cairo_stroke(cr);

  cairo_set_dash(cr, dashed2, len2, 1);

  cairo_move_to(cr, 40, 50);  
  cairo_line_to(cr, 200, 50);
  cairo_stroke(cr);

  cairo_set_dash(cr, dashed3, 1, 0);

  cairo_move_to(cr, 40, 70);  
  cairo_line_to(cr, 200, 70);
  cairo_stroke(cr);  
}
```
In this example, we will draw three lines with different dash patterns.

```cpp
static const double dashed1[] = {4.0, 21.0, 2.0};
```
We have a pattern of three numbers. We have 4 points drawn, 21 not drawn and 2 drawn. Then 4 points not drawn, 21 points drawn and 2 not drawn. This pattern takes turns until the end of the line.

```cpp
static int len1  = sizeof(dashed1) / sizeof(dashed1[0]);
```
We get the size of the array.

```cpp
cairo_set_dash(cr, dashed1, len1, 0);
```
We set the dash.

```cpp
static const double dashed3[] = {1.0};
...
cairo_set_dash(cr, dashed3, 1, 0);

cairo_move_to(cr, 40, 70);  
cairo_line_to(cr, 200, 70);
cairo_stroke(cr);
```
These lines create a line with a pen dash of a symmetric pattern of alternating single on and off points.

(Dashes) Figure: Dashes

## Line caps
The line caps are endpoints of lines.

- CAIRO_LINE_CAP_SQUARE
- CAIRO_LINE_CAP_ROUND
- CAIRO_LINE_CAP_BUTT

There are three different line cap styles in Cairo.

(Line caps) Figure: Square, round and butt caps

A line with a CAIRO_LINE_CAP_SQUARE cap will have a different size, than a line with a CAIRO_LINE_CAP_BUTT cap. If a line is width px wide, the line with a CAIRO_LINE_CAP_SQUARE cap will be exactly width px greater in size. width/2 px at the beginning and width/2 px at the end.

```cpp
static void do_drawing(cairo_t *cr)
{
  cairo_set_line_width(cr, 10);

  cairo_set_line_cap(cr, CAIRO_LINE_CAP_BUTT); 
  cairo_move_to(cr, 30, 50); 
  cairo_line_to(cr, 150, 50);
  cairo_stroke(cr);

  cairo_set_line_cap(cr, CAIRO_LINE_CAP_ROUND); 
  cairo_move_to(cr, 30, 90); 
  cairo_line_to(cr, 150, 90);
  cairo_stroke(cr);

  cairo_set_line_cap(cr, CAIRO_LINE_CAP_SQUARE); 
  cairo_move_to(cr, 30, 130); 
  cairo_line_to(cr, 150, 130);
  cairo_stroke(cr);

  cairo_set_line_width(cr, 1.5);

  cairo_move_to(cr, 30, 40);  
  cairo_line_to(cr, 30, 140);
  cairo_stroke(cr);

  cairo_move_to(cr, 150, 40);  
  cairo_line_to(cr, 150, 140);
  cairo_stroke(cr);

  cairo_move_to(cr, 155, 40);  
  cairo_line_to(cr, 155, 140);
  cairo_stroke(cr);    
}
```
The example draws three lines with three different caps. It will also graphically demonstrate the differences is size of the lines.

```cpp
cairo_set_line_width(cr, 10);
```
Our lines will be 10 px wide.

```cpp
cairo_set_line_cap(cr, CAIRO_LINE_CAP_ROUND); 
cairo_move_to(cr, 30, 90); 
cairo_line_to(cr, 150, 90);
cairo_stroke(cr);
```
Here we draw a horizontal line with a CAIRO_LINE_CAP_ROUND cap.

```cpp
cairo_set_line_width(cr, 1.5);

cairo_move_to(cr, 30, 40);  
cairo_line_to(cr, 30, 140);
cairo_stroke(cr);
```

This is one of the three vertical lines used to demostrate the differences in size.

(Line caps) Figure: Line caps

## Line joins
The lines can be joined using three different join styles:

- CAIRO_LINE_JOIN_BEVEL
- CAIRO_LINE_JOIN_ROUND
- CAIRO_LINE_JOIN_MITER

(Bevel, Round, Miter line joins) Figure: Bevel, Round, Miter line joins

CAIRO_LINE_JOIN_BEVEL uses a cut-off join, where the join is cut off at half the line width from the joint point. CAIRO_LINE_JOIN_ROUND uses a rounded join, where the center of the circle is the joint point. CAIRO_LINE_JOIN_MITER uses a sharp, angled corner.

```cpp
static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgb(cr, 0.1, 0, 0);

  cairo_rectangle(cr, 30, 30, 100, 100);
  cairo_set_line_width(cr, 14);
  cairo_set_line_join(cr, CAIRO_LINE_JOIN_MITER); 
  cairo_stroke(cr);

  cairo_rectangle(cr, 160, 30, 100, 100);
  cairo_set_line_width(cr, 14);
  cairo_set_line_join(cr, CAIRO_LINE_JOIN_BEVEL); 
  cairo_stroke(cr);

  cairo_rectangle(cr, 100, 160, 100, 100);
  cairo_set_line_width(cr, 14);
  cairo_set_line_join(cr, CAIRO_LINE_JOIN_ROUND); 
  cairo_stroke(cr);    
}
```
In this example, we draw three thick rectangles with various line joins.

```cpp
cairo_rectangle(cr, 30, 30, 100, 100);
cairo_set_line_width(cr, 14);
cairo_set_line_join(cr, CAIRO_LINE_JOIN_MITER); 
cairo_stroke(cr);
```

In this code example, we draw a rectangle with CAIRO_LINE_JOIN_MITER join style. The lines are 14px wide.

(Line joins) Figure: Line joins

In this chapter, we did some basic drawing.