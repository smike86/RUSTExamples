use std::sync::Mutex;
use std::thread;

fn main() {
	let m = Mutex::new(5);
	{
		let mut num = m.lock().unwrap();
		*num += 1;
	}
	println!("m = {}", m.lock().unwrap());
}
