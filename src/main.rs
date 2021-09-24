use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

	let (tx, rx) = mpsc::channel();

	let tx1 = mpsc::Sender::clone(&tx);
	thread::spawn(move || {
    	let vals = vec![
        	String::from("hola"),
        	String::from("desde"),
        	String::from("el"),
        	String::from("hilo 1"),
    	];

    	for val in vals {
        	tx1.send(val).unwrap();
        	thread::sleep(Duration::from_secs(1));
    	}
	});

	thread::spawn(move || {
    	let vals = vec![
        	String::from("vemos"),
        	String::from("como "),
        	String::from("trabaja"),
        	String::from("hilo 2"),
    	];

    	for val in vals {
        	tx.send(val).unwrap();
        	thread::sleep(Duration::from_secs(1));
    	}
	});

	for received in rx {
    	println!("Obetener mensaje {}", received);
	}

}

