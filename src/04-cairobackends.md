# Cairo backends
last modified July 13, 2020

The Cairo library supports various backends. In this section of the Cairo graphics tutorial, we will use Cairo to create a PNG image, PDF file, SVG file and we will draw on a GTK window.

## PNG image
In the first example, we will create a PNG image.

```cpp
#include <cairo.h>

int main(void)
{
  cairo_surface_t *surface;
  cairo_t *cr;

  surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 390, 60);
  cr = cairo_create(surface);

  cairo_set_source_rgb(cr, 0, 0, 0);
  cairo_select_font_face(cr, "Sans", CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_NORMAL);
  cairo_set_font_size(cr, 40.0);

  cairo_move_to(cr, 10.0, 50.0);
  cairo_show_text(cr, "Disziplin ist Macht.");

  cairo_surface_write_to_png(surface, "image.png");

  cairo_destroy(cr);
  cairo_surface_destroy(surface);

  return 0;
}
```
This example is a small console application that will create a PNG image.

```cpp
#include <cairo.h>
```
In this header file, we will find declarations of our functions and constants.

```cpp
cairo_surface_t *surface;
cairo_t *cr;
```
Here we declare a surface and a Cairo context.

```cpp
surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 390, 60);
cr = cairo_create(surface);
```
We create a surface and a Cairo context. The surface is an 390x60 px image.

```cpp
cairo_set_source_rgb(cr, 0, 0, 0);
```
We will draw in black ink.

```cpp
cairo_select_font_face(cr, "Sans", CAIRO_FONT_SLANT_NORMAL,
    CAIRO_FONT_WEIGHT_NORMAL);
cairo_set_font_size(cr, 40.0);
```
We choose a font type and set its size.

```cpp
cairo_move_to(cr, 10.0, 50.0);
cairo_show_text(cr, "Disziplin ist Macht.");
```
We move to a (10.0, 50.0) position within the image and draw the text.

```cpp
cairo_surface_write_to_png(surface, "image.png");
```
This function call creates the PNG image.

```cpp
cairo_destroy(cr);
cairo_surface_destroy(surface);
```
In the end, we clean the resources.

## PDF file
In the second example, we will use the Cairo library to create a simple PDF file.

```cpp
#include <cairo.h>
#include <cairo-pdf.h>

int main(void) 
{
  cairo_surface_t *surface;
  cairo_t *cr;

  surface = cairo_pdf_surface_create("pdffile.pdf", 504, 648);
  cr = cairo_create(surface);

  cairo_set_source_rgb(cr, 0, 0, 0);
  cairo_select_font_face (cr, "Sans", CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_NORMAL);
  cairo_set_font_size (cr, 40.0);

  cairo_move_to(cr, 10.0, 50.0);
  cairo_show_text(cr, "Disziplin ist Macht.");

  cairo_show_page(cr);

  cairo_surface_destroy(surface);
  cairo_destroy(cr);

  return 0;
}
```
We must open the PDF file in a PDF viewer. Linux users can use KPDF or Evince viewers.

```cpp
surface = cairo_pdf_surface_create("pdffile.pdf", 504, 648);
```
To render a PDF file, we must create a PDF surface using the cairo_pdf_surface_create() function call. The size of the PDF file is specified in points, which is a standard in typesetting.

```cpp
cairo_show_page(cr);
```
The cairo_show_page() finishes rendering of the PDF file.

(PDF file in Evince) Figure: PDF file in Evince

## SVG file
The next example creates a simple SVG (Scalable Vector Graphics) file. The SVG is one of the hottest technologies these days.

```cpp
#include <cairo.h>
#include <cairo-svg.h> 
 
int main(void) 
{
  cairo_surface_t *surface;
  cairo_t *cr;

  surface = cairo_svg_surface_create("svgfile.svg", 390, 60);
  cr = cairo_create(surface);

  cairo_set_source_rgb(cr, 0, 0, 0);
  cairo_select_font_face(cr, "Sans", CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_NORMAL);
  cairo_set_font_size(cr, 40.0);

  cairo_move_to(cr, 10.0, 50.0);
  cairo_show_text(cr, "Disziplin ist Macht.");

  cairo_surface_destroy(surface);
  cairo_destroy(cr);

  return 0;
}
```
We can use Firefox, Opera or Inkscape programs to open the svgfile.svg file.

```cpp
surface = cairo_svg_surface_create("svgfile.svg", 390, 60);
```
To create a SVG file in Cairo, we must create a svg surface using the cairo_svg_surface_create() function call.

```cpp
cr = cairo_create(surface);
```
A Cairo context is created from a SVG surface.

The Rest of the code is identical to the previous examples.

(SVG file in Chrome) SVG file in Chrome

## GTK Window
In the last example, we will draw on the GTK window. This backend will be used throughout the rest of the tutorial.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

static void do_drawing(cairo_t *);

static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);

  return FALSE;
}

static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgb(cr, 0, 0, 0);
  cairo_select_font_face(cr, "Sans", CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_NORMAL);
  cairo_set_font_size(cr, 40.0);

  cairo_move_to(cr, 10.0, 50.0);
  cairo_show_text(cr, "Disziplin ist Macht.");    
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
  gtk_window_set_default_size(GTK_WINDOW(window), 400, 90); 
  gtk_window_set_title(GTK_WINDOW(window), "GTK window");

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
The example pops up a centered GTK window, on which we draw the "Disziplin ist Macht" text.

```cpp
#include <cairo.h>
#include <gtk/gtk.h>
```
We include the necessary Cairo and GTK headers.

```cpp
static gboolean on_draw_event(GtkWidget *widget, cairo_t *cr, 
    gpointer user_data)
{      
  do_drawing(cr);

  return FALSE;
}
We delegate the actual drawing to the do_drawing() function. The parameter sent is the Cairo context.

static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgb(cr, 0, 0, 0);
  cairo_select_font_face(cr, "Sans", CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_NORMAL);
  cairo_set_font_size(cr, 40.0);

  cairo_move_to(cr, 10.0, 50.0);
  cairo_show_text(cr, "Disziplin ist Macht.");    
}
```
The Cairo functions perform the drawing.

```cpp
darea = gtk_drawing_area_new();
gtk_container_add(GTK_CONTAINER(window), darea);
```
We create a GtkDrawingArea widget and add it to the container window. It is used for custom drawing.

```cpp
g_signal_connect(G_OBJECT(darea), "draw", 
    G_CALLBACK(on_draw_event), NULL); 
```
When the GtkDrawingArea widget needs to be redrawn, it emits the draw signal. We connect that signal to the on_draw_event() callback.

(GTK window) Figure: GTK window

In this chapter we have covered supported Cairo backends.