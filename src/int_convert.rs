pub fn i64_to_i16_panic(inp: i64) -> i16 {
	if inp > std::i16::MAX as i64 || inp < std::i16::MIN as i64 {
		panic!("[Blueprint convert error] Number convert overflow {}", inp);
	} else {
		inp as i16
	}
}
pub fn u64_to_u32_panic(inp: u64) -> u32 {
	if inp > std::u32::MAX as u64 {
		panic!("[Blueprint convert error] Cord overflow {}", inp);
	} else {
		inp as u32
	}
}
pub fn u64_to_u16_panic(inp: u64) -> u16 {
	if inp > std::u16::MAX as u64 {
		panic!("[Blueprint convert error] Cord overflow {}", inp);
	} else {
		inp as u16
	}
}