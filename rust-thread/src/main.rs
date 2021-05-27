use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default)]
struct Mesh {
    pub v: Vec<Vertex>,
    pub i: Vec<i32>,
}

struct Chunk {
    pub id: (i32, i32, i32),
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            id: (0, 0, 0)
        }
    }
}

fn main() {
    let (txworker, rxmain): (Sender<Mesh>, Receiver<Mesh>) = mpsc::channel();
    let (txmain, rxworker): (Sender<Chunk>, Receiver<Chunk>) = mpsc::channel();

    let child = thread::spawn(move || {
        loop {
            let chunk  = rxworker.recv().unwrap();
            if chunk.id.0 == 99 {
                break ();
            } else {
                txworker.send(Mesh::default()).unwrap();
            }
        }
    });
    for i in 0..100 {
        txmain.send(Chunk { id: (i, 0, 0) }).unwrap();
    }
    for i in 0..99 {
        rxmain.recv().unwrap();
    }
    child.join().unwrap();
}
