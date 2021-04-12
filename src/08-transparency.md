# Transparency
last modified July 13, 2020

In this part of the Cairo C API tutorial, we will talk about transparency. We will provide some basic definitions and two interesting transparency effects.

Transparency is the quality of being able to see through a material. The easiest way to understand transparency is to imagine a piece of glass or water. Technically, the rays of light can go through the glass and this way we can see objects behind the glass.

In computer graphics, we can achieve transparency effects using alpha compositing. Alpha compositing is the process of combining an image with a background to create the appearance of partial transparency. The composition process uses an alpha channel. Alpha channel is an 8-bit layer in a graphics file format that is used for expressing translucency (transparency). The extra eight bits per pixel serves as a mask and represents 256 levels of translucency.

## Transparent rectangles
The first example will draw ten rectangles with different levels of transparency.

```cpp
static void do_drawing(cairo_t *cr)
{
  gint i;
  for ( i = 1; i <= 10; i++) {
      cairo_set_source_rgba(cr, 0, 0, 1, i*0.1);
      cairo_rectangle(cr, 50*i, 20, 40, 40);
      cairo_fill(cr);  
  }      
}
```
The cairo_set_source_rgba() has an optional alpha parameter to provide transparency. This code creates ten rectangles with alpha values from 0.1 ... 1.

(Transparency) Figure: Transparency

## Puff effect
In the following example, we create a puff effect. The example will display a growing centered text that will gradually fade out from some point. This is a very common effect which we can often see in flash animations. The cairo_paint_with_alpha() method is crucial to create the effect.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

void do_drawing(cairo_t *, GtkWidget *);

struct {
  gboolean timer; 
  gdouble alpha;
  gdouble size;
} glob;

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr, widget);

  return FALSE;
}

void do_drawing(cairo_t *cr, GtkWidget *widget)
{
  cairo_text_extents_t extents;

  GtkWidget *win = gtk_widget_get_toplevel(widget);
  
  gint width, height;
  gtk_window_get_size(GTK_WINDOW(win), &width, &height);  
  
  gint x = width/2;
  gint y = height/2;
  
  cairo_set_source_rgb(cr, 0.5, 0, 0); 
  cairo_paint(cr);   

  cairo_select_font_face(cr, "Courier",
      CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_BOLD);
 
  glob.size += 0.8;

  if (glob.size > 20) {
      glob.alpha -= 0.01;
  }

  cairo_set_font_size(cr, glob.size);
  cairo_set_source_rgb(cr, 1, 1, 1); 

  cairo_text_extents(cr, "ZetCode", &extents);
  cairo_move_to(cr, x - extents.width/2, y);
  cairo_text_path(cr, "ZetCode");
  cairo_clip(cr);

  cairo_paint_with_alpha(cr, glob.alpha);
  
  if (glob.alpha <= 0) {
      glob.timer = FALSE;
  }     
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
  
  glob.timer = TRUE;
  glob.alpha = 1.0;
  glob.size = 1.0;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL); 
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);
 
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 200); 
  gtk_window_set_title(GTK_WINDOW(window), "Puff");

  g_timeout_add(14, (GSourceFunc) time_handler, (gpointer) window);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example creates a growing and fading text on the window.

```cpp
struct {
  gboolean timer; 
  gdouble alpha;
  gdouble size;
} glob; 
```
Here we define some variables inside a structure. This is used to avoid using globals.

```cpp
draw_text(cr, widget); 
```

The actual drawing of the text is delegated to the draw_text() function.

```cpp
GtkWidget *win = gtk_widget_get_toplevel(widget);

gint width, height;
gtk_window_get_size(GTK_WINDOW(win), &width, &height);  

gint x = width/2;
gint y = height/2;
```
The text is going to be centered on the window. Therefore we need to find out the size of the parent widget.

```cpp
cairo_set_source_rgb(cr, 0.5, 0, 0); 
cairo_paint(cr); 
```
The background of the window is filled with some dark red colour.

```cpp
cairo_select_font_face(cr, "Courier",
    CAIRO_FONT_SLANT_NORMAL,
    CAIRO_FONT_WEIGHT_BOLD);
```
The text is going to be in Courier bold font.

```cpp
glob.size += 0.8;

if (glob.size > 20) {
    glob.alpha -= 0.01;
}
```
The size of the text is increased by 0.8 units. After it has reached 20 units, the alpha value starts decreasing. And the text fades away slowly.

```cpp
cairo_text_extents(cr, "ZetCode", &extents);
cairo_move_to(cr, x - extents.width/2, y);
```
We get the text metrics. We will use only the text width. We move to a position where the text will be centered on the window.

```cpp
cairo_text_path(cr, "ZetCode");
cairo_clip(cr);

cairo_paint_with_alpha(cr, glob.alpha);
```
We get the path of the text with the cairo_text_path() method. We restrict the painting to the current path using the cairo_clip() method. The cairo_paint_with_alpha() method paints the current source everywhere within the current clip region using a mask of the alpha value.

```cpp
glob.timer = TRUE;
glob.alpha = 1.0;
glob.size = 1.0;
```
We initiate three variables.

```cpp
static gboolean time_handler(GtkWidget *widget)
{
  if (!glob.timer) return FALSE;

  gtk_widget_queue_draw(widget);

  return TRUE;
}
```
The main function of the time_handler call is to redraw the window regularly. When the function returns FALSE, the timeout function will cease to work.

```cpp
g_timeout_add(14, (GSourceFunc) time_handler, (gpointer) window);
```
We create a timer function. This function will call time_handler every 14 ms.

(Puff effect) Figure: Puff effect

## Waiting demo
In this examle, we use transparency effect to create a waiting demo. We will draw 8 lines that will gradually fade out creating an illusion, that a line is moving. Such effects are often used to inform users that a lengthy task is going on behind the scenes. An example is streaming video over the Internet.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>
#include <math.h>

static void do_drawing(cairo_t *, GtkWidget *);

struct {
  gushort count;
} glob;

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr, widget);

  return FALSE;
}

static void do_drawing(cairo_t *cr, GtkWidget *widget)
{  
  static gdouble const trs[8][8] = {
      { 0.0, 0.15, 0.30, 0.5, 0.65, 0.80, 0.9, 1.0 },
      { 1.0, 0.0,  0.15, 0.30, 0.5, 0.65, 0.8, 0.9 },
      { 0.9, 1.0,  0.0,  0.15, 0.3, 0.5, 0.65, 0.8 },
      { 0.8, 0.9,  1.0,  0.0,  0.15, 0.3, 0.5, 0.65},
      { 0.65, 0.8, 0.9,  1.0,  0.0,  0.15, 0.3, 0.5 },
      { 0.5, 0.65, 0.8, 0.9, 1.0,  0.0,  0.15, 0.3 },
      { 0.3, 0.5, 0.65, 0.8, 0.9, 1.0,  0.0,  0.15 },
      { 0.15, 0.3, 0.5, 0.65, 0.8, 0.9, 1.0,  0.0, }
  };

  GtkWidget *win = gtk_widget_get_toplevel(widget);
  
  gint width, height;
  gtk_window_get_size(GTK_WINDOW(win), &width, &height);

  cairo_translate(cr, width/2, height/2);

  gint i = 0;
  for (i = 0; i < 8; i++) {
      cairo_set_line_width(cr, 3);
      cairo_set_line_cap(cr, CAIRO_LINE_CAP_ROUND);
      cairo_set_source_rgba(cr, 0, 0, 0, trs[glob.count%8][i]);

      cairo_move_to(cr, 0.0, -10.0);
      cairo_line_to(cr, 0.0, -40.0);
      cairo_rotate(cr, M_PI/4);

      cairo_stroke(cr);
  }   
}

static gboolean time_handler(GtkWidget *widget)
{
  glob.count += 1;
  gtk_widget_queue_draw(widget);
  
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
  gtk_container_add(GTK_CONTAINER (window), darea);

  g_signal_connect(G_OBJECT(darea), "draw", 
      G_CALLBACK(on_draw_event), NULL);  
  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 250, 150); 
  gtk_window_set_title(GTK_WINDOW(window), "Waiting demo");

  g_timeout_add(100, (GSourceFunc) time_handler, (gpointer) window);
  gtk_widget_show_all(window);  

  gtk_main();

  return 0;
}
```
We draw eight lines with eight different alpha values.

```cpp
static gdouble const trs[8][8] = {
     { 0.0, 0.15, 0.30, 0.5, 0.65, 0.80, 0.9, 1.0 },
     { 1.0, 0.0,  0.15, 0.30, 0.5, 0.65, 0.8, 0.9 },
     { 0.9, 1.0,  0.0,  0.15, 0.3, 0.5, 0.65, 0.8 },
     { 0.8, 0.9,  1.0,  0.0,  0.15, 0.3, 0.5, 0.65},
     { 0.65, 0.8, 0.9,  1.0,  0.0,  0.15, 0.3, 0.5 },
     { 0.5, 0.65, 0.8, 0.9, 1.0,  0.0,  0.15, 0.3 },
     { 0.3, 0.5, 0.65, 0.8, 0.9, 1.0,  0.0,  0.15 },
     { 0.15, 0.3, 0.5, 0.65, 0.8, 0.9, 1.0,  0.0, }
};
```
This is a two dimensional array of transparency values used in this demo. There are 8 rows, each for one state. Each of the 8 lines will continuosly use these values.

```cpp
cairo_set_line_width(cr, 3);
cairo_set_line_cap(cr, CAIRO_LINE_CAP_ROUND);
```
We make the lines a bit thicker, so that they are better visible. We draw the lines with rouded caps.

```cpp
cairo_set_source_rgba(cr, 0, 0, 0, trs[glob.count%8][i]);
```
Here we define the transparency value for a line.

```cpp
cairo_move_to(cr, 0.0, -10.0);
cairo_line_to(cr, 0.0, -40.0);
cairo_rotate(cr, M_PI/4);
```
These code will draw each of the eight lines.

```cpp
g_timeout_add(100, (GSourceFunc) time_handler, (gpointer) window);
```
We use a timer function to create animation.

(Waiting demo) Figure: Waiting demo

In this part of the Cairo tutorial, we have covered transparency.