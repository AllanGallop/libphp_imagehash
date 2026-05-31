use ext_php_rs::prelude::*;

mod hashing;
mod index;

pub use index::ImageHashIndex;

#[php_function]
pub fn hamming_distance(a: String, b: String) -> u32 {
    hashing::hamming_distance_hex(a, b)
}

#[php_function]
pub fn image_dhash(path: String) -> String {
    hashing::image_dhash_hex(path)
}

#[php_function]
pub fn image_phash(path: String) -> String {
    hashing::image_phash_hex(path)
}

#[php_function]
pub fn imagehash_version() -> &'static str {
    "php_imagehash 1.1.0"
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<ImageHashIndex>()
        .function(wrap_function!(hamming_distance))
        .function(wrap_function!(image_dhash))
        .function(wrap_function!(image_phash))
        .function(wrap_function!(imagehash_version))
}