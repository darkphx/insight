use crate::blueprint;
use crate::vec_multiple_mutable;
//use std::collections::HashMap;
use std::time::Instant;

const MAX_INPUTS:usize=1000;

#[derive(Debug)]
pub struct Crunch{
	pub logic_blocks_count:usize,
	pub logic_blocks_types:Box<[(blueprint::LogicType, bool)]>,
	pub logic_blocks_inputs:Box<[[usize;MAX_INPUTS]]>,
	pub logic_blocks_inputs_length:Box<[usize]>,
	pub cache_save: bool,
	pub cache_states: [Box<[bool]>;2],
}

pub fn init(bp: &mut blueprint::Blueprint)->Crunch{
	//println!("lol");
	blueprint::blueprint_check_panic(bp);
	let logic_blocks_count:usize=bp.blocks_logic.len();
	let mut logic_blocks_types:Vec<(blueprint::LogicType, bool)>=Vec::with_capacity(logic_blocks_count);
	let mut logic_blocks_inputs:Vec<[usize;MAX_INPUTS]>=Vec::with_capacity(logic_blocks_count);
	let mut logic_blocks_inputs_length:Vec<usize>=Vec::with_capacity(logic_blocks_count);

	for a in &bp.blocks_logic {
		let e:blueprint::LogicType = match a.logic_type.0{
			blueprint::LogicType::AND=>blueprint::LogicType::AND,
			blueprint::LogicType::OR =>blueprint::LogicType::OR,
			blueprint::LogicType::XOR=>blueprint::LogicType::XOR,
		};
		let d: (blueprint::LogicType, bool)=(e,a.logic_type.1);
		logic_blocks_types.insert(a.id, d);
		let mut b:[usize;MAX_INPUTS]=[0;MAX_INPUTS];
		let mut c=0;

		for i in &a.inputs {
			if !bp.block_logic_deleted[*i] {
				if c==MAX_INPUTS{
					panic!("[calculate|cruch] block with id {} has at least{} inputs, only {} are supported",a.id,c,MAX_INPUTS);
				}
				b[c]=*i;
				c+=1;
			}
		}
		logic_blocks_inputs.insert(a.id, b);
		logic_blocks_inputs_length.insert(a.id,c);
	}

	Crunch{
		logic_blocks_count:logic_blocks_count,
		logic_blocks_types:logic_blocks_types.into_boxed_slice(),
		logic_blocks_inputs:logic_blocks_inputs.into_boxed_slice(),
		logic_blocks_inputs_length:logic_blocks_inputs_length.into_boxed_slice(),
		cache_save: false,
		cache_states: [vec![false;logic_blocks_count].into_boxed_slice(),vec![false;logic_blocks_count].into_boxed_slice()]
	}
}

pub fn calculate_logic(chr: &mut Crunch) {
	//let y = vec_multiple_mutable::index_twice(&mut bp.cache_states, if bp.cache_save{1}else{0}, if bp.cache_save{0}else{1});
	let d:&mut Box<[bool]>;
	let e:&mut Box<[bool]>;
	unsafe {
		d=&mut *(chr.cache_states.get_unchecked_mut(if chr.cache_save{1}else{0}) as *mut _);
		e=&mut *(chr.cache_states.get_unchecked_mut(if chr.cache_save{0}else{1}) as *mut _);
	}
			for a in 0..chr.logic_blocks_count {

				e[a]= block_logic_calculate(a,&*d,/*e,*/&chr.logic_blocks_types,&chr.logic_blocks_inputs,&chr.logic_blocks_inputs_length);
			}
	chr.cache_save = !chr.cache_save;
}

fn block_logic_calculate(
	id:usize,
	cache_old:&Box<[bool]>,
	//cache_new:&mut Box<[bool]>,
	types:&Box<[(blueprint::LogicType, bool)]>,
	inputs:&Box<[[usize;MAX_INPUTS]]>,
	inputs_length:&Box<[usize]>,
)->bool {
	//let now = Instant::now();
	//println!("{} {}",inputs.len(),id);
	//cache_new[id] = 
/*	println!("----  {:?}",inputs_length[id]);
	for i in 0..inputs_length[id] {
		println!("{:?}",cache_old[inputs[id][i]]);
	}*/
	if inputs_length[id] == 0 {
		false
	} else {
		let out = match types[id].0 {
			blueprint::LogicType::AND => {
				let mut out = true;
				for i in 0..inputs_length[id] {
					if cache_old[inputs[id][i]] == false {
						out = false;
						break;
					}
				}
				out
			}

			blueprint::LogicType::OR => {
				let mut out = false;
				for i in 0..inputs_length[id] {
					if cache_old[inputs[id][i]] == true {
						out = true;
						break;
					}
				}
				out
			}

			blueprint::LogicType::XOR => {
				let mut ammount: usize = 0;
				for i in 0..inputs_length[id] {
					if cache_old[inputs[id][i]] == true {
						ammount+=1;
					}
				}
				ammount % 2 == 1
			}
		};
		if types[id].1{
			!out
		}else{
			out
		}
	}
}
