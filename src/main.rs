mod encode;
mod decode;
use encode::encode;
use decode::decode;

pub fn main() {
    let pngs = encode("./file.jpg");

    decode(&pngs, "./out.jpg");
}
