// use crate::Point;
// use super::{Config, Options, OpSet, ResolvedOptions, Drawable, SVGNS, RoughGenerator };

// pub struct RoughSVG {
//   gen: RoughGenerator,
//   svg: SVGSVGElement,
// }

// impl RoughSVG {
//   pub fn new(svg: SVGSVGElement, config: Config) -> Self {
//     Self {
//       svg,
//       gen: RoughGenerator::new(config)
//     }
//   }

//   pub fn draw(&self, drawable: Drawable) -> SVGGElement {
//     // let sets = drawable.sets || [];
//     // let o = drawable.options || self.getDefaultOptions();
//     // let doc = self.svg.ownerDocument || window.document;
//     // let g = doc.createElementNS(SVGNS, 'g');
//     // for (let drawing of sets) {
//     //   let path = null;
//     //   switch (drawing.type) {
//     //     case 'path': {
//     //       path = doc.createElementNS(SVGNS, 'path');
//     //       path.setAttribute('d', self.opsToPath(drawing));
//     //       path.setAttribute('stroke', o.stroke);
//     //       path.setAttribute('stroke-width', o.strokeWidth + '');
//     //       path.setAttribute('fill', 'none');
//     //       if (o.strokeLineDash) {
//     //         path.setAttribute('stroke-dasharray', o.strokeLineDash.join(' ').trim());
//     //       }
//     //       if (o.strokeLineDashOffset) {
//     //         path.setAttribute('stroke-dashoffset', `${o.strokeLineDashOffset}`);
//     //       }
//     //       break;
//     //     }
//     //     case 'fillPath': {
//     //       path = doc.createElementNS(SVGNS, 'path');
//     //       path.setAttribute('d', self.opsToPath(drawing));
//     //       path.setAttribute('stroke', 'none');
//     //       path.setAttribute('stroke-width', '0');
//     //       path.setAttribute('fill', o.fill || '');
//     //       if (drawable.shape === 'curve' || drawable.shape === 'polygon') {
//     //         path.setAttribute('fill-rule', 'evenodd');
//     //       }
//     //       break;
//     //     }
//     //     case 'fillSketch': {
//     //       path = self.fillSketch(doc, drawing, o);
//     //       break;
//     //     }
//     //   }
//     //   if (path) {
//     //     g.appendChild(path);
//     //   }
//     // }
//     // return g;
//   }

//   fn fill_sketch(&self, doc: Document, drawing: OpSet, o: ResolvedOptions) -> SVGPathElement {
//     // let fweight = o.fillWeight;
//     // if (fweight < 0) {
//     //   fweight = o.strokeWidth / 2;
//     // }
//     // let path = doc.createElementNS(SVGNS, 'path');
//     // path.setAttribute('d', self.opsToPath(drawing));
//     // path.setAttribute('stroke', o.fill || '');
//     // path.setAttribute('stroke-width', fweight + '');
//     // path.setAttribute('fill', 'none');
//     // if (o.fillLineDash) {
//     //   path.setAttribute('stroke-dasharray', o.fillLineDash.join(' ').trim());
//     // }
//     // if (o.fillLineDashOffset) {
//     //   path.setAttribute('stroke-dashoffset', `${o.fillLineDashOffset}`);
//     // }
//     // return path;
//   }

//   pub fn get_generator(&self) -> RoughGenerator {
//     // self.gen
//     unimplemented!()
//   }

//   pub fn get_default_options(&self) -> ResolvedOptions {
//     // self.gen.defaultOptions
//     unimplemented!()
//   }

//   pub fn ops_to_path(&self, drawing: OpSet) -> String {
//     // self.gen.opsToPath(drawing)
//     unimplemented!()
//   }

//   pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, options: Options) -> SVGGElement {
//     // let d = self.gen.line(x1, y1, x2, y2, options);
//     // self.draw(d)
//     unimplemented!()
//   }

//   pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64, options: Options) -> SVGGElement {
//     // let d = self.gen.rectangle(x, y, width, height, options);
//     // self.draw(d)
//     unimplemented!()
//   }

//   pub fn ellipse(&self, x: f64, y: f64, width: f64, height: f64, options: Options) -> SVGGElement {
//     // let d = self.gen.ellipse(x, y, width, height, options);
//     // self.draw(d)
//     unimplemented!()
//   }

//   pub fn circle(&self, x: f64, y: f64, diameter: f64, options: Options) -> SVGGElement {
//     let d = self.gen.circle(x, y, diameter, options);
//     self.draw(d)
//   }

//   pub fn linear_path(&self, points: Vec<Point>, options: Options) -> SVGGElement {
//     let d = self.gen.linearPath(points, options);
//     self.draw(d)
//   }

//   pub fn polygon(&self, points: Vec<Point>, options: Options) -> SVGGElement {
//     let d = self.gen.polygon(points, options);
//     self.draw(d)
//   }

//   pub fn arc(&self, x: f64, y: f64, width: f64, height: f64, start: f64, stop: f64, closed: bool, options: Options) -> SVGGElement {
//     // closed: boolean = false
//     let d = self.gen.arc(x, y, width, height, start, stop, closed, options);
//     self.draw(d)
//   }

//   pub fn curve(&self, points: Vec<Point>, options: Options) -> SVGGElement {
//     let d = self.gen.curve(points, options);
//     self.draw(d)
//   }

//   pub fn path(&self, d: &str, options: Options) -> SVGGElement {
//     let drawing = self.gen.path(d, options);
//     self.draw(drawing)
//   }
// }