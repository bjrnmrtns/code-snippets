use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    let (txworker, rxmain): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let (txmain, rxworker): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let child = thread::spawn(move || {
        loop {
            let i = rxworker.recv().unwrap();
            if i == 99 {
                break ();
            } else {
                txworker.send(i).unwrap();
            }
        }
    });
    for i in 0..100 {
        txmain.send(i).unwrap();
    }
    for i in 0..99 {
        println!("{}", rxmain.recv().unwrap());
    }
    child.join().unwrap();
}
