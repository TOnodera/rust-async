use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        let res = tx.send(val);
        match res {
            Ok(_) => {}
            Err(_) => {
                println!("send message failure.")
            }
        }

        let recieved = rx.recv();
        match recieved {
            Ok(message) => {
                println!("{}", message);
            }
            Err(_) => {
                println!("message recieved failure.");
            }
        }
    });
}
