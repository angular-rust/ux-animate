# Gradients
last modified July 13, 2020

In this part of the Cairo graphics tutorial, we will cover gradients. We will mention linear and radial gradients.

In computer graphics, gradient is a smooth blending of shades from light to dark or from one colour to another. In 2D drawing programs and paint programs, gradients are used to create colourful backgrounds and special effects as well as to simulate lights and shadows. (answers.com)

## Linear gradients
Linear gradients are blendings of colours or shades of colours along a line. They are created with the cairo_pattern_create_linear() function.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

void draw_gradient1(cairo_t *);
void draw_gradient2(cairo_t *);
void draw_gradient3(cairo_t *);

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{         
  draw_gradient1(cr);
  draw_gradient2(cr);
  draw_gradient3(cr);  

  return FALSE;
}

void draw_gradient1(cairo_t *cr)
{
  cairo_pattern_t *pat1;  
  pat1 = cairo_pattern_create_linear(0.0, 0.0,  350.0, 350.0);

  gdouble j;
  gint count = 1;
  for ( j = 0.1; j < 1; j += 0.1 ) {
      if (( count % 2 ))  {
          cairo_pattern_add_color_stop_rgb(pat1, j, 0, 0, 0);
      } else { 
          cairo_pattern_add_color_stop_rgb(pat1, j, 1, 0, 0);
      }
   count++;
  }

  cairo_rectangle(cr, 20, 20, 300, 100);
  cairo_set_source(cr, pat1);
  cairo_fill(cr);  
  
  cairo_pattern_destroy(pat1);
}

void draw_gradient2(cairo_t *cr)
{
  cairo_pattern_t *pat2;
  pat2 = cairo_pattern_create_linear(0.0, 0.0,  350.0, 0.0);

  gdouble i;
  gint count = 1;
  for ( i = 0.05; i < 0.95; i += 0.025 ) {
      if (( count % 2 ))  {
          cairo_pattern_add_color_stop_rgb(pat2, i, 0, 0, 0);
      } else { 
          cairo_pattern_add_color_stop_rgb(pat2, i, 0, 0, 1);
      }
   count++;
  }

  cairo_rectangle(cr, 20, 140, 300, 100);
  cairo_set_source(cr, pat2);
  cairo_fill(cr);  
  
  cairo_pattern_destroy(pat2);
}

void draw_gradient3(cairo_t *cr)
{
  cairo_pattern_t *pat3;
  pat3 = cairo_pattern_create_linear(20.0, 260.0, 20.0, 360.0);

  cairo_pattern_add_color_stop_rgb(pat3, 0.1, 0, 0, 0);
  cairo_pattern_add_color_stop_rgb(pat3, 0.5, 1, 1, 0);
  cairo_pattern_add_color_stop_rgb(pat3, 0.9, 0, 0, 0);

  cairo_rectangle(cr, 20, 260, 300, 100);
  cairo_set_source(cr, pat3);
  cairo_fill(cr);  
  
  cairo_pattern_destroy(pat3);
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
  gtk_window_set_default_size(GTK_WINDOW(window), 340, 390); 
  gtk_window_set_title(GTK_WINDOW(window), "Linear gradients");

  gtk_widget_set_app_paintable(window, TRUE);
  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example draws three rectangles filled with linear gradients.

```cpp
pat3 = cairo_pattern_create_linear(20.0, 260.0, 20.0, 360.0);
```
Here we create a linear gradient pattern. The parameters specify the line, along which we draw the gradient. In our case it is a vertical line.

```cpp
cairo_pattern_add_color_stop_rgb(pat3, 0.1, 0, 0, 0);
cairo_pattern_add_color_stop_rgb(pat3, 0.5, 1, 1, 0);
cairo_pattern_add_color_stop_rgb(pat3, 0.9, 0, 0, 0);
```
We define colour stops to produce our gradient pattern. In this case, the gradient is a blending of black and yellow colours. By adding two black and one yellow stops, we create a horizontal gradient pattern. What these stops actually mean? In our case, we begin with black colour, which will stop at 1/10 of the size. Then we begin to gradually paint in yellow, which will culminate at the centre of the shape. The yellow colour stops at 9/10 of the size, where we begin painting in black again, until the end.

(Linear gradients) Figure: Linear gradients

## Radial gradients
Radial gradients are blendings of colours or shades of colours between two circles. The cairo_pattern_create_radial() function s is used to create radial gradients in Cairo.

```cpp
#include <cairo.h>
#include <math.h>
#include <gtk/gtk.h>

void draw_gradient1(cairo_t *);
void draw_gradient2(cairo_t *);


static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{         
  draw_gradient1(cr);
  draw_gradient2(cr); 

  return FALSE;
}

void draw_gradient1(cairo_t *cr)
{
  cairo_pattern_t *r1; 
    
  cairo_set_source_rgba(cr, 0, 0, 0, 1);
  cairo_set_line_width(cr, 12);  
  cairo_translate(cr, 60, 60);
  
  r1 = cairo_pattern_create_radial(30, 30, 10, 30, 30, 90);
  cairo_pattern_add_color_stop_rgba(r1, 0, 1, 1, 1, 1);
  cairo_pattern_add_color_stop_rgba(r1, 1, 0.6, 0.6, 0.6, 1);
  cairo_set_source(cr, r1);
  cairo_arc(cr, 0, 0, 40, 0, M_PI * 2);
  cairo_fill(cr);
         
  cairo_pattern_destroy(r1);
}

void draw_gradient2(cairo_t *cr)
{
  cairo_pattern_t *r2; 
  
  cairo_translate(cr, 120, 0);
  
  r2 = cairo_pattern_create_radial(0, 0, 10, 0, 0, 40);  
  cairo_pattern_add_color_stop_rgb(r2, 0, 1, 1, 0);
  cairo_pattern_add_color_stop_rgb(r2, 0.8, 0, 0, 0);
  cairo_set_source(cr, r2);
  cairo_arc(cr, 0, 0, 40, 0, M_PI * 2);
  cairo_fill(cr);     
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
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200); 
  gtk_window_set_title(GTK_WINDOW(window), "Radial gradients");

  gtk_widget_set_app_paintable(window, TRUE);
  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
In the example, we draw two radial gradients.

```cpp
r1 = cairo_pattern_create_radial(30, 30, 10, 30, 30, 90);
cairo_pattern_add_color_stop_rgba(r1, 0, 1, 1, 1, 1);
cairo_pattern_add_color_stop_rgba(r1, 1, 0.6, 0.6, 0.6, 1);
cairo_set_source(cr, r1);
cairo_arc(cr, 0, 0, 40, 0, M_PI * 2);
cairo_fill(cr);
```
We draw a circle and fill its inside with a radial gradient. The radial gradient is defined by two circles. The cairo_pattern_add_color_stop_rgba() function defines the colours. We can experiment with the position of the circles or the length of their radius. In the first gradient example, we have created an object which resembles a 3D shape.

```cpp
r2 = cairo_pattern_create_radial(0, 0, 10, 0, 0, 40);  
cairo_pattern_add_color_stop_rgb(r2, 0, 1, 1, 0);
cairo_pattern_add_color_stop_rgb(r2, 0.8, 0, 0, 0);
cairo_set_source(cr, r2);
cairo_arc(cr, 0, 0, 40, 0, M_PI * 2);
cairo_fill(cr);
```
In this example, the circles that define the radial gradient and the custom drawn circle have a common center point.

(Radial gradients) Figure: Radial gradients

In this chapter of the Cairo graphics tutorial we have covered gradients.