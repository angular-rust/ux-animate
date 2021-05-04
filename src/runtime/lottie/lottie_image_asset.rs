use std::fmt;
// use crate::Image;// not available in wasm

pub struct LottieImageAsset {
    width: i32,
    height: i32,
    id: String,
    file_name: String,
    dir_name: String,
    // loaded_image: Image,
}

impl fmt::Display for LottieImageAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LottieImageAsset(width: {}, height: {}, id: {}, fileName: {}, dirName: {})",
            self.width, self.height, self.id, self.file_name, self.dir_name
        )
    }
}
