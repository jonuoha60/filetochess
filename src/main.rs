mod encode;
mod decode;
mod utils;
use encode::encode;
use decode::decode;

pub fn main() {
    let pngs = encode("./img.jpg");

    decode(&pngs, "./out.jpg");
}
