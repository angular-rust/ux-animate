use std::collections::HashMap;

use crate::runtime::lottie::composition::LottieComposition;

pub trait LottieProvider {
    fn load(&self);
}

pub struct LottieCache {
    maximum_size: usize,
    cache: HashMap<String, LottieComposition>,
}

impl LottieCache {
    pub fn new(maximum_size: Option<usize>) -> Self {
        let maximum_size = maximum_size.unwrap_or(1000);

        Self {
            maximum_size,
            cache: Default::default(),
        }
    }

    pub fn put_if_absent(&self, key: &str, load: LottieComposition) -> LottieComposition {
        unimplemented!()
    }

    fn check_cache_size(&self) {}

    fn clear(&self) {}
}
