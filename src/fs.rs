use std::fs::File;
use std::io::Read;
pub fn read_string(address: &str) -> String {
	let mut file = File::open(address).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	data
}
