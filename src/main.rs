extern crate compression;

fn main() {
	use std::io::{ Cursor, Read };
	use compression::brotli::Decompressor;
	use compression::bitreader::BitReader;

	let brotli_stream = BitReader::new(Cursor::new(vec![
		0x0b, 0x00, 0x80, 0x58, 0x03,
	]));

	let mut decompressed = &mut String::new();
	let _ = Decompressor::new(brotli_stream).read_to_string(&mut decompressed);

	assert_eq!("", decompressed);

	println!("{:?}", decompressed);
}

