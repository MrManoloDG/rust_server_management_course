use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
    	for i in 1..10 {
        	println!("hola {} desde el primer hilo", i);
        	thread::sleep(Duration::from_millis(10));
    	}
	});

	let handle1 = thread::spawn(|| {
    	for i in 1..10 {
        	println!("hola {} desde el segundo hilo", i);
        	thread::sleep(Duration::from_millis(10));
    	}
	});
    
	let handle2 = thread::spawn(|| {
    	for i in 1..10 {
        	println!("hola {} desde el tercer hilo", i);
        	thread::sleep(Duration::from_millis(10));
    	}
	});

    handle.join().unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();
}

