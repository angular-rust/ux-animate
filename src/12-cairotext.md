# Text in Cairo
last modified July 13, 2020

In this part of the Cairo graphics tutorial, we will work with text.

## Soulmate
In the first example, we will display some lyrics on the GTK+ window.

```cpp
static void do_drawing(cairo_t *cr)
{
  cairo_set_source_rgb(cr, 0.1, 0.1, 0.1); 

  cairo_select_font_face(cr, "Purisa",
      CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_BOLD);

  cairo_set_font_size(cr, 13);

  cairo_move_to(cr, 20, 30);
  cairo_show_text(cr, "Most relationships seem so transitory");  
  cairo_move_to(cr, 20, 60);
  cairo_show_text(cr, "They're all good but not the permanent one");

  cairo_move_to(cr, 20, 120);
  cairo_show_text(cr, "Who doesn't long for someone to hold");

  cairo_move_to(cr, 20, 150);
  cairo_show_text(cr, "Who knows how to love you without being told");
  cairo_move_to(cr, 20, 180);
  cairo_show_text(cr, "Somebody tell me why I'm on my own");
  cairo_move_to(cr, 20, 210);
  cairo_show_text(cr, "If there's a soulmate for everyone");    
}
```
In this example, we display part of the lyrics from the Natasha Bedingfield's Soulmate song.

```cpp
cairo_select_font_face(cr, "Purisa",
   CAIRO_FONT_SLANT_NORMAL,
   CAIRO_FONT_WEIGHT_BOLD);
```
Here we select the font face. The function takes three parameters, the font family, font slant and the font weight.

```cpp
cairo_set_font_size(cr, 13);
```
Here we specify the font size.

```cpp
cairo_move_to(cr, 20, 30);
cairo_show_text(cr, "Most relationships seem so transitory"); 
```
We display the text on the window by specifying the position of the text and calling the cairo_show_text() function.

(Soulmate) Figure: Soulmate

## Centered text
Next we will show, how to center text on the window.

```cpp
static void do_drawing(cairo_t *cr, GtkWidget *widget)
{
  cairo_text_extents_t extents;
  
  GtkWidget *win = gtk_widget_get_toplevel(widget);
  
  gint w, h;
  gtk_window_get_size(GTK_WINDOW(win), &w, &h);    

  cairo_select_font_face(cr, "Courier",
      CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_BOLD);

  cairo_set_font_size(cr, 60);
  
  cairo_text_extents(cr, "ZetCode", &extents);

  cairo_move_to(cr, w/2 - extents.width/2, h/2);  
  cairo_show_text(cr, "ZetCode");    
}
```
The code will center a text on the window. It remains centered, even if we resize the window.

```cpp
GtkWidget *win = gtk_widget_get_toplevel(widget);

gint w, h;
gtk_window_get_size(GTK_WINDOW(win), &w, &h);
```
To center a text on the window, it is necessary to get the size of of the parent window.

```cpp
cairo_select_font_face(cr, "Courier",
    CAIRO_FONT_SLANT_NORMAL,
    CAIRO_FONT_WEIGHT_BOLD);

cairo_set_font_size(cr, 60);
```
We select a font and its size to be displayed.

```cpp
cairo_text_extents(cr, "ZetCode", &extents);
```
We get the text extents. These are some numbers that describe the text. We need the width of the text for our example.

```cpp
cairo_move_to(cr, w/2 - extents.width/2, h/2);  
cairo_show_text(cr, "ZetCode");
```
We position the text into the middle of the window and show it using the cairo_show_text() method.

(Centered text) Figure: Centered text

## Shaded text
Now we will show a shaded text on the window.

```cpp
static void do_drawing(cairo_t *cr, GtkWidget *widget)
{
  cairo_select_font_face(cr, "Serif", CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_BOLD);
  cairo_set_font_size(cr, 50);

  cairo_set_source_rgb(cr, 0, 0, 0);
  cairo_move_to(cr, 40, 60);  
  cairo_show_text(cr, "ZetCode");  
  
  cairo_set_source_rgb(cr, 0.5, 0.5, 0.5);
  cairo_move_to(cr, 43, 63);  
  cairo_show_text(cr, "ZetCode");    
}
```
To create a shade, we draw the text twice. In different colours. The second text is moved a bit to the right and bottom.

```cpp
cairo_set_source_rgb(cr, 0, 0, 0);
cairo_move_to(cr, 40, 60);  
cairo_show_text(cr, "ZetCode"); 
```
The first text is drawn in black ink. It serves as a shade.

```cpp
cairo_set_source_rgb(cr, 0.5, 0.5, 0.5);
cairo_move_to(cr, 43, 63);  
cairo_show_text(cr, "ZetCode");
```
The second text is drawn in some gray ink. It is moved by 3px to the right and to the bottom.

(Shaded text) Figure: Shaded text

## Text filled with gradient
The following example will create a nice effect. We will fill a text with some linear gradient.

```cpp
static void do_drawing(cairo_t *cr, GtkWidget *widget)
{  
  cairo_pattern_t *pat; 
  
  cairo_set_source_rgb(cr, 0.2, 0.2, 0.2);
  cairo_paint(cr);
  
  gint h = 90;
  
  cairo_select_font_face(cr, "Serif", CAIRO_FONT_SLANT_ITALIC, 
      CAIRO_FONT_WEIGHT_BOLD);
  cairo_set_font_size(cr, h);
  
  pat = cairo_pattern_create_linear(0, 15, 0, h*0.8);
  cairo_pattern_set_extend(pat, CAIRO_EXTEND_REPEAT);
  cairo_pattern_add_color_stop_rgb(pat, 0.0, 1, 0.6, 0);
  cairo_pattern_add_color_stop_rgb(pat, 0.5, 1, 0.3, 0);
                  
  cairo_move_to(cr, 15, 80);
  cairo_text_path(cr, "ZetCode");
  cairo_set_source(cr, pat);
  cairo_fill(cr);
}
```
We draw a text on the window filled with a linear gradient. The colours are some orange colours.

```cpp
cairo_set_source_rgb(cr, 0.2, 0.2, 0.2);
cairo_paint(cr);
```
To make it more visually appealing, we paint the background in dark gray colour.

```cpp
pat = cairo_pattern_create_linear(0, 15, 0, h*0.8);
cairo_pattern_set_extend(pat, CAIRO_EXTEND_REPEAT);
cairo_pattern_add_color_stop_rgb(pat, 0.0, 1, 0.6, 0);
cairo_pattern_add_color_stop_rgb(pat, 0.5, 1, 0.3, 0);
```
The linear gradient is created.

```cpp
cairo_move_to(cr, 15, 80);
cairo_text_path(cr, "ZetCode");
cairo_set_source(cr, pat);
cairo_fill(cr);
```
The text is displayed on the window. We use the gradient as a source for painting.

(Text filled with gradient) Figure: Text filled with gradient

## Glyphs
The cairo_show_text() method is only suitable for simple text rendering. Cairo developers call it a toy method. More professional text rendering is done with glyphs. A glyph is a graphic symbol which provides a form for a character. A character provides a meaning. It can have multiple glyphs. A character has no intrinsic appearance. A glyph has no intrinsic meaning.

Note that many common programming requirements conserning text are addressed by the Pango library.

```cpp
static void do_drawing(cairo_t *cr, GtkWidget *widget)
{    
  cairo_select_font_face(cr, "Serif", CAIRO_FONT_SLANT_NORMAL,
      CAIRO_FONT_WEIGHT_NORMAL);
  cairo_set_font_size(cr, 13);

  const int n_glyphs = 20 * 35;
  cairo_glyph_t glyphs[n_glyphs];

  gint i = 0;  
  gint x, y;
  
  for (y=0; y<20; y++) {
      for (x=0; x<35; x++) {
          glyphs[i] = (cairo_glyph_t) {i, x*15 + 20, y*18 + 20};
          i++;
      }
  }
  
  cairo_show_glyphs(cr, glyphs, n_glyphs);
} 
```
This code shows 700 glyphs of a chosen font.

```cpp
const int n_glyphs = 20 * 35;
cairo_glyph_t glyphs[n_glyphs];
```
The glyphs array will store three integer values. The first value is the index of the glyph to the chosen font type. The second and the third values are x, y positions of a glyph.

```cpp
cairo_show_glyphs(cr, glyphs, n_glyphs);
```
The cairo_show_glyphs() method shows the glyphs on the window.

This chapter covered text in Cairo.