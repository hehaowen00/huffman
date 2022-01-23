use huffman::codec::Codec;

fn main() {
    let msg = "Hello, World!";
    let (dict, encoded) = Codec::encode(msg.as_bytes());

    println!("original length (bits): {}", msg.len() * 8);
    println!("encoded length (bits): {}\n", encoded.len());

    let decoded = Codec::decode(dict, &encoded);
    let decoded_str = std::str::from_utf8(&decoded).unwrap();

    println!("encoded: {:?}", encoded.as_bytes());
    println!("decoded: {}", decoded_str);
}
