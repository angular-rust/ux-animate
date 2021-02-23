#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod animatables;
use animatables::*;

mod animations;
use animations::*;

mod composition;
use composition::*;

mod drawing;
use drawing::*;

mod elements;
use elements::*;

mod images;
use images::*;

mod layers;
use layers::*;

mod mathutils;
use mathutils::*;

mod painting;
use painting::*;

mod utils;
use utils::*;

mod values;
use values::*;


// stubs
pub struct CompositionLayer;
pub struct AnimationController;
pub struct Canvas;
pub struct CustimPainter;
pub struct Size;

// stubs

pub struct Lottie {
    pub composition: LottieComposition
}

impl Lottie {
    fn new() -> Self {
        Self {
            composition: Default::default()
        }
    }

    pub fn create_state(&self) -> LottieState {
        LottieState::new(self.composition.clone())
    }
}

pub struct LottieState {
    composition: LottieComposition,
    composition_layer: Option<CompositionLayer>,
    animation: Option<AnimationController>
}

impl LottieState {
    pub fn new(composition: LottieComposition) -> Self {
        Self {
            composition,
            composition_layer: None,
            animation: None
        }
    }

    pub fn init_state() {
        unimplemented!()
    }

    pub fn handle_change() {
        unimplemented!()
    }

    pub fn build() {
        unimplemented!()
    }

    pub fn dispose() {
        unimplemented!()
    }
}

pub struct LottiePainter {
    composition_layer: CompositionLayer,
    scale: f64,
    alpha: i64
}

impl LottiePainter {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn paint(canvas: Canvas, size: Size) {
        unimplemented!()
    }

    pub fn should_repaint(old_delegate: CustimPainter) -> bool {
        unimplemented!()
    }
}