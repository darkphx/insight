use std::collections::HashMap;
use crate::int_convert;

#[derive(Debug)]
pub struct Cords {
	pub x: i16,
	pub y: i16,
	pub z: i16,
}

#[derive(Debug)]
pub struct Blueprint {
	pub blocks_normal: Vec<BlockNormal>,
	pub blocks_logic: Vec<BlockLogic>,
	pub latest_block_normal_id: usize,
	pub latest_block_logic_id: usize,
	pub cache_save: bool,
	pub cache_logic: [Vec<bool>;2],
	pub block_logic_deleted: Vec<bool>,
}

#[derive(Debug)]
pub enum BlockNormalSize{
	Some(Cords),
	None,
}

#[derive(Debug)]
pub struct BlockNormal {
	pub deleted: bool,
	pub start: Cords,
	pub id: usize,

	pub size: BlockNormalSize,
	pub type_id: String,
}

#[derive(Debug)]
pub struct BlockLogic {
	pub start: Cords,
	pub id: usize,

	pub logic_type: (LogicType, bool),
	pub inputs: Vec<usize>,
}

#[derive(Debug)]
pub enum LogicType {
	AND,
	OR,
	XOR,
}

pub fn parse(data: &str) -> Blueprint {
	let json: serde_json::Value = serde_json::from_str(data).unwrap();

	let blueprint_version: u64 = json["version"].as_u64().unwrap();

	let mut blocks_logic: Vec<BlockLogic> = Vec::new();
	let mut latest_block_logic_id: usize = 0;
	let mut blocks_normal: Vec<BlockNormal> = Vec::new();
	let mut latest_block_normal_id: usize = 0;

	match blueprint_version {
		3 => {
			let a = json["bodies"].as_array().unwrap();
			if a.len() != 1 {
				panic!(
					"Cant handle blueprint with {} bodies; can only handle 1",
					a.len()
				);
			}
			let b = a[0]["childs"].as_array().unwrap();
			let mut blocks_logic_id: HashMap<u32 /* SM id */, usize /* id */> = HashMap::new();
			let mut logic_connections: Vec<(u32 /* SM id reciever*/, usize /* sender */)> = Vec::new();
			for c in b {
				//println!("c: {:?}\n",c);
				let shape_id: &str = c["shapeId"].as_str().unwrap();
				let start_position: Cords = json_bounds_to_struct(&c["pos"]);
				match shape_id {
					"9f0f56e8-2c31-4d83-996c-d00a9b296c3f" => {
						blocks_logic.insert(latest_block_logic_id, 
							BlockLogic {
								id: latest_block_logic_id,
								start: start_position,
								inputs: Vec::new(), // is filled later in code
								logic_type: logic_id_to_enum(
									c["controller"].as_object().unwrap()["mode"]
										.as_u64()
										.unwrap(),
								),
							},
						);
						blocks_logic_id.insert(
							int_convert::u64_to_u32_panic(
								c["controller"].as_object().unwrap()["id"].as_u64().unwrap(),
							),
							latest_block_logic_id,
						);
						let f = &c["controller"].as_object().unwrap()["controllers"];
						if f.is_null() == false {
							let d = f.as_array().unwrap();
							for e in d {
								logic_connections.push((
									int_convert::u64_to_u32_panic(e["id"].as_u64().unwrap()), // sm id reciever
									latest_block_logic_id,                                           // sender
								));
							}
						}
						latest_block_logic_id+=1;
					}
					"8f7fd0e7-c46e-4944-a414-7ce2437bb30f" => {
						println!("[Blueprint convert] Timers not supported yet {:?}",start_position);
					}
					"a6c6ce30-dd47-4587-b475-085d55c6a3b4" |
					"8aedf6c2-94e1-4506-89d4-a0227c552f1e" => {
						blocks_normal.insert(
							latest_block_normal_id,
							BlockNormal {
								deleted: false,
								id: latest_block_normal_id,
								start: start_position,
								size: if c["bounds"].is_null(){BlockNormalSize::None}else{BlockNormalSize::Some(json_bounds_to_struct(&c["bounds"]))},
								type_id: String::from(shape_id),
							},
						);
						latest_block_normal_id += 1;
					}
					_ => {
						//panic!("[Blueprint convert error] Unknown block! id {}", shape_id);
						println!("[Blueprint convert] Unknown block {} {:?} {}",shape_id,start_position, c);
						blocks_normal.insert(
							latest_block_normal_id,
							BlockNormal {
								deleted: false,
								id: latest_block_normal_id,
								start: start_position,
								size: if c["bounds"].is_null(){BlockNormalSize::None}else{BlockNormalSize::Some(json_bounds_to_struct(&c["bounds"]))},
								type_id: String::from(shape_id),
							},
						);
						latest_block_normal_id += 1;
					}
				}
			}
			for i in logic_connections {
				let a = blocks_logic_id.get(&i.0);
				match a {
					Some(b) => {
						if *b>=blocks_logic.len() {
							panic!("[Blueprint convert error] ");
						}
						blocks_logic[*b].inputs.push(i.1);
					}
					None => {
						println!("dropped connection");
				//		panic!("[Blueprint convert error] Logic Connection to not existing id {}", i.0);
					}
				}
			}
			//println!("{}",json["bodies"]);
			Blueprint {
				blocks_logic: blocks_logic,
				blocks_normal: blocks_normal,
				latest_block_logic_id: latest_block_logic_id,
				latest_block_normal_id: latest_block_normal_id,
				cache_save: false,
				cache_logic: [vec![false;latest_block_logic_id],vec![false;latest_block_logic_id]],
				block_logic_deleted: vec![false;latest_block_logic_id],
			}
		}
		_ => {
			panic!(
				"[Blueprint convert error] Blueprint version {} not supported (only version 3 is supported)",
				blueprint_version
			)
		}
	}
}
fn json_bounds_to_struct(bounds: &serde_json::Value) -> Cords {
	Cords {
		x: int_convert::i64_to_i16_panic(bounds["x"].as_i64().unwrap()),
		y: int_convert::i64_to_i16_panic(bounds["y"].as_i64().unwrap()),
		z: int_convert::i64_to_i16_panic(bounds["z"].as_i64().unwrap()),
	}
}
fn logic_id_to_enum(inp: u64) -> (LogicType, bool) {
	match inp {
		0 => (LogicType::AND, false),
		1 => (LogicType::OR, false),
		2 => (LogicType::XOR, false),
		3 => (LogicType::AND, true),
		4 => (LogicType::OR, true),
		5 => (LogicType::XOR, true),
		_ => {
			panic!("[Blueprint convert error] Unknown logic type {}", inp)
		}
	}
}

fn logic_enum_to_id(inp: LogicType, inverted: bool) -> u64 {
	let mut out: u64 = match inp {
		LogicType::AND => 0,
		LogicType::OR => 1,
		LogicType::XOR => 3,
	};
	if inverted == true {
		out += 3;
	}
	out
}

pub fn blueprint_check_panic(bp:&Blueprint){
	for a in 0..2{
		if bp.cache_logic[a].len() != bp.blocks_logic.len() {
			panic!(
				"[calculate|check] logic cache {} wrong size (has size {}, size needed: {})",
				a,
				bp.cache_logic[a].len(),
				bp.blocks_logic.len()
			);
		}
	}
	if bp.block_logic_deleted.len() != bp.blocks_logic.len() {
		panic!(
			"[calculate|check] deleted cache wrong size (has size {}, size needed: {})",
			bp.block_logic_deleted.len(),
			bp.blocks_logic.len()
		);
	}
}