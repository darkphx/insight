use crate::blueprint;
use crate::vec_multiple_mutable;
//use std::collections::HashMap;
use std::time::Instant;

pub fn calculate_logic(bp: &mut blueprint::Blueprint) {
	blueprint::blueprint_check_panic(bp);
	//let y = vec_multiple_mutable::index_twice(&mut bp.cache_states, if bp.cache_save{1}else{0}, if bp.cache_save{0}else{1});
	let d;
	let e;
	unsafe {
		d=&mut *(bp.cache_logic.get_unchecked_mut(if bp.cache_save{1}else{0}) as *mut _);
		e=&mut *(bp.cache_logic.get_unchecked_mut(if bp.cache_save{0}else{1}) as *mut _);
	}
			for a in &bp.blocks_logic {
				if !bp.block_logic_deleted[a.id] {
					//println!("\nBlock: {:?}",b);

					block_logic_calculate(&a, &*d, e, &bp.block_logic_deleted);
				}
			}
	bp.cache_save = !bp.cache_save;
}

fn block_logic_calculate(
	input: &blueprint::BlockLogic,
	cache_old: &Vec<bool>,
	cache_new: &mut Vec<bool>,
	blocks_logic_deleted: &Vec<bool>,
) {
	//let now = Instant::now();
	let mut inputs: Vec<bool> = Vec::with_capacity(input.inputs.len());
	for i in &input.inputs {
		if !blocks_logic_deleted[*i] {
			inputs.push(cache_old[*i]);
		}
	}
	let output: bool = if inputs.len() == 0 {
		false
	} else {
		let out = match input.logic_type.0 {
			blueprint::LogicType::AND => {
				let mut out = true;
				for i in inputs {
					if i == false {
						out = false;
					}
				}
				out
			}

			blueprint::LogicType::OR => {
				let mut out = false;
				for i in inputs {
					if i == true {
						out = true;
						break;
					}
				}
				out
			}

			blueprint::LogicType::XOR => {
				let mut ammount: u16 = 0;
				for i in inputs {
					if i == true {
						ammount += 1;
					}
				}
				ammount % 2 == 1
			}
		};
		out != input.logic_type.1
	};
	cache_new[input.id] = output;
	//println!("Gateout {}",output);
	//input.output_cache[{if cache{1}else{0}}] = output;
	/*for i in input.connects_to.iter(){
	}*/
	//println!("op time: {:?}", now.elapsed());
}
