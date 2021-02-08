#![allow(dead_code)]
#![allow(unused_imports)]

extern crate serde;
extern crate serde_json;
use std::time::{Instant};
mod blueprint;
mod fs;
mod calculace_dyn;
mod calculace_crunch;
mod vec_multiple_mutable;
mod int_convert;
use std::{thread, time};
use clap::clap_app;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "darkphx ( @dark_#0368 )")
        (about: "ScrapMechanic blueprint logic emulator")
        (@arg BLUEPRINTFILE: -f --file +takes_value "which blueprint file to emulate")
        (@arg WAITTIME: -s --sleep +takes_value "wait between ticks in ms")
	).get_matches();
	if let None = matches.value_of("BLUEPRINTFILE") {
		println!("\n\nyou need to specify a blueprint file with -f filepath \n\n");
		panic!("you need to specify a blueprint file with -f filepath ");
	}
	let mut waittime_ms:u64=1000;
	if let Some(a) = matches.value_of("WAITTIME") {
		waittime_ms=a.to_string().parse::<u64>().unwrap();
	}
	let mut bp = blueprint::parse(fs::read_string(matches.value_of("BLUEPRINTFILE").unwrap()).as_str());
	//println!("{:?}",bp);
	/*let mut x0:u64;
	let mut z0:(f64,u64)=(0.0,0);*/
	/*let mut x1:u64;
	let mut z1:(f64,u64)=(0.0,0);*/
	let mut chr=calculace_crunch::init(&mut bp);
	//println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n{:?}",chr);
	loop {
		/*x0=0;
		let now = Instant::now();
		for _ in 0..10001{
			calculace_dyn::calculate_logic(&mut bp);
		//	calculace_crunch::calculate_logic(&mut chr);
			x0+=1;
			//println!("{:?}",bp.cache_states[{if bp.cache_save{1}else{0}}]);
		}
		let y=1.0/(now.elapsed().as_micros() as f64/x0 as f64/1000000.0);
		z0.0+=y;
		z0.1+=1;
		println!("dyn    tps: {:?} average: {:?}", y,z0.0/z0.1 as f64);*/



		chrout(&chr);
		//x1=0;
		//let now = Instant::now();
		for _ in 0..10001{
			//calculace_dyn::calculate_logic(&mut bp);
			calculace_crunch::calculate_logic(&mut chr);
			//x1+=1;
			chrout(&chr);
			thread::sleep(time::Duration::from_millis(waittime_ms));
			
		}
		/*let y=1.0/(now.elapsed().as_micros() as f64/x1 as f64/1000000.0);
		z1.0+=y;
		z1.1+=1;
		println!("crunch tps: {:?} average: {:?}", y,z1.0/z1.1 as f64);*/
	}
}

fn chrout(chr:&calculace_crunch::Crunch){
	println!("\n\n\n\n");
	let mut a:u64=0;
	let mut b:u64=0;
	let mut c:u64=0;
	for i in chr.cache_states[{if chr.cache_save{1}else{0}}].iter(){
		print!("{}",if *i {1}else{0});
		c+=1;
		if c>200 {
			println!("");
			c=0;
		}
	}
	for i in 0..chr.cache_states[{if chr.cache_save{1}else{0}}].len(){
		if chr.cache_states[{if chr.cache_save{1}else{0}}][i]!=chr.cache_states[{if chr.cache_save{0}else{1}}][i]{
			a+=1;
		}
		b+=1;
	}
	println!("\nchanged {} of {}    {}%",a,b,a as f64/b as f64 *100.0);
}