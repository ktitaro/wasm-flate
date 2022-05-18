use std::io::prelude::*;
use wasm_bindgen::prelude::*;
use flate2::Compression;
use flate2::read::{GzEncoder, ZlibEncoder, DeflateEncoder};
use flate2::write::{GzDecoder, ZlibDecoder, DeflateDecoder};

#[wasm_bindgen]
pub fn gzip_encode(data: &str) -> String {
    let input = data.as_bytes();
    let mut buffer = Vec::new();
    let mut encoder = GzEncoder::new(input, Compression::default());
    encoder.read_to_end(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[wasm_bindgen]
pub fn gzip_decode(data: &str) -> String {
   let input = data.as_bytes();
   let mut buffer = Vec::new();
   let mut decoder = GzDecoder::new(buffer);
   decoder.write_all(&input).unwrap();
   decoder.try_finish().unwrap();
   buffer = decoder.finish().unwrap();
   String::from_utf8(buffer).unwrap()
}

#[wasm_bindgen]
pub fn zlib_encode(data: &str) -> String {
    let input = data.as_bytes();
    let mut buffer = Vec::new();
    let mut encoder = ZlibEncoder::new(input, Compression::default());
    encoder.read_to_end(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[wasm_bindgen]
pub fn zlib_decode(data: &str) -> String {
   let input = data.as_bytes();
   let mut buffer = Vec::new();
   let mut decoder = ZlibDecoder::new(buffer);
   decoder.write_all(&input).unwrap();
   decoder.try_finish().unwrap();
   buffer = decoder.finish().unwrap();
   String::from_utf8(buffer).unwrap()
}

#[wasm_bindgen]
pub fn deflate_encode(data: &str) -> String {
    let input = data.as_bytes();
    let mut buffer = Vec::new();
    let mut encoder = DeflateEncoder::new(input, Compression::default());
    encoder.read_to_end(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[wasm_bindgen]
pub fn deflate_decode(data: &str) -> String {
   let input = data.as_bytes();
   let mut buffer = Vec::new();
   let mut decoder = DeflateDecoder::new(buffer);
   decoder.write_all(&input).unwrap();
   decoder.try_finish().unwrap();
   buffer = decoder.finish().unwrap();
   String::from_utf8(buffer).unwrap()
}