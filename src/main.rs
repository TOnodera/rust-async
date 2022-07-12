use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        Duration::from_secs(5);
        let val = String::from("hi");
        let res = tx.send(val);
        match res {
            Ok(_) => {}
            Err(_) => {
                println!("send message failure.")
            }
        }
    });

    loop {
        let recieved = rx.try_recv();
        match recieved {
            Ok(message) => {
                println!("{}", message);
                break;
            }
            Err(_) => {
                println!("please wait for a seconds...");
            }
        }
    }
}
