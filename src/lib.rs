use std::io::prelude::*;
use wasm_bindgen::prelude::*;
use flate2::read::{GzDecoder, ZlibDecoder, DeflateDecoder};
use flate2::write::{GzEncoder, ZlibEncoder, DeflateEncoder};
use flate2::Compression;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn gzip_encode(data: &str) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    base64::encode(&compressed_bytes)
}

#[wasm_bindgen]
pub fn gzip_decode(data: &str) -> String {
    let compressed_bytes = base64::decode(&data).unwrap();
    let mut decoder = GzDecoder::new(&compressed_bytes[..]);
    let mut buffer = String::new();
    decoder.read_to_string(&mut buffer).unwrap();
    return buffer;
}

#[wasm_bindgen]
pub fn zlib_encode(data: &str) -> String {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_bytes = encoder.finish();
    base64::encode(&compressed_bytes.unwrap())
}

#[wasm_bindgen]
pub fn zlib_decode(data: &str) -> String {
    let compressed_bytes = base64::decode(&data).unwrap();
    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut buffer = String::new();
    decoder.read_to_string(&mut buffer).unwrap();
    return buffer;
}

#[wasm_bindgen]
pub fn deflate_encode(data: &str) -> String {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    base64::encode(&compressed_bytes)
}

#[wasm_bindgen]
pub fn deflate_decode(data: &str) -> String {
    let compressed_bytes = base64::decode(&data).unwrap();
    let mut decoder = DeflateDecoder::new(&compressed_bytes[..]);
    let mut buffer = String::new();
    decoder.read_to_string(&mut buffer).unwrap();
    return buffer;
}
