use animate::lottie::Lottie;
use animate::prelude::*;

fn main() {
    let _lottie = Lottie::construct()
        .asset("assets/Tests/Shapes.json")
        .width(230.0)
        .height(230.0)
        .animate(true)
        .build()
        .unwrap_or_default();
}
