# The Cairo graphics library
last modified July 13, 2020

Welcome to the Cairo graphics tutorial. This tutorial will teach you basics and some advanced topics of the Cairo 2D vector drawing library. In most examples we will use the GTK+ programming library. This tutorial is done in C programming language.

## 2D Vector Graphics
There are two different computer graphics. Vector and raster graphics. Raster graphics represents images as a collection of pixels. Vector graphics is the use of geometrical primitives such as points, lines, curves or polygons to represent images. These primitives are created using mathematical equations.

Both types of computer graphics have advantages and disadvantages. The advantages of vector graphics over raster are:

smaller size
ability to zoom indefinitely
moving, scaling, filling or rotating does not degrade the quality of an image

## Cairo
Cairo is a library for creating 2D vector graphics. It is written in the C programming language. Bindings for other computer languages exist. Python, Perl, C++, C#, Java. Cairo is a multiplatform library, it works on Linux, BSDs, and Mac OS.

Cairo supports various backends.

- X Window System
- Win32 GDI
- Mac OS X Quartz
- PNG
- PDF
- PostScript
- SVG

This means that we can use the library to draw on windows on Linux/BSDs, Windows, Mac OS and we can use the library to create PNG images, PDF files, PostScript files, and SVG files.

We can compare the cairo library to the GDI+ library on Windows OS and the Quartz 2D on Mac OS. Cairo is an open source software library. From version 2.8, the cairo library is part of the GTK+ system.

## Compiling examples
The examples are created in the C programming language. We use the GNU C compiler to compile them.

```bash
gcc example.c -o example `pkg-config --cflags --libs gtk+-3.0` 
```

Note that the order of compiling options is important.