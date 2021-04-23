use crate::runtime::lottie::{
    composition::LottieComposition, lottie_image_asset::LottieImageAsset,
};
// use crate::Image; // not available in wasm
use super::LottieProvider;

#[derive(Default, Clone)]
pub struct AssetLottie {}

impl AssetLottie {
    pub fn new(asset_name: &str) -> Self {
        unimplemented!()
    }

    // fn load_image(composition: LottieComposition, lottie_image: LottieImageAsset) -> Image {
    //     unimplemented!()
    // }
}

impl LottieProvider for AssetLottie {
    fn load(&self) {}
}
