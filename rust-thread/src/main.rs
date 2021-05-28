use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

enum MeshLoadResult {
    Quit,
    Loaded(Mesh)
}

struct Mesh {
    pub asset_id: u32,
}

impl Mesh {
    pub fn new(id: u32) -> Self {
        Self {
            asset_id: id
        }
    }
}

enum ChunkLoadCommand {
    Quit,
    Load(u32)
}

#[derive(Default)]
struct MeshRegistry {
    vs: Vec<Mesh>
}

#[derive(Default)]
struct Assets {
    pub assets: Vec<u32>,
}

impl Assets {
    pub fn add(&mut self, asset: u32) {
        self.assets.push(asset);
    }
}

impl MeshRegistry {
    pub fn add(&mut self, mesh: Mesh) {
        self.vs.push(mesh);
    }
}

fn main() {
    let mut mesh_registry = MeshRegistry::default();
    let mut assets = Assets::default();
    assets.add(0);
    assets.add(1);
    assets.add(2);
    let (txworker, rxmain): (Sender<MeshLoadResult>, Receiver<MeshLoadResult>) = mpsc::channel();
    let (txmain, rxworker): (Sender<ChunkLoadCommand>, Receiver<ChunkLoadCommand>) = mpsc::channel();

    let child = thread::spawn(move || {
        loop {
            match rxworker.recv().unwrap() {
                ChunkLoadCommand::Quit => {txworker.send(MeshLoadResult::Quit); return ();},
                ChunkLoadCommand::Load(asset_id) => {
                    if let Some(asset_id) = assets.assets.iter().find(|x| **x == asset_id) {
                        txworker.send(MeshLoadResult::Loaded(Mesh::new(*asset_id)));
                    }
                }
            }
        }
    });
    for i in 0..100 {
        txmain.send(ChunkLoadCommand::Load(i)).unwrap();
    }
    txmain.send(ChunkLoadCommand::Quit).unwrap();
    loop {
        std::thread::sleep( Duration::from_millis(100));
        if let Ok(mesh_result) = rxmain.try_recv() {
            match mesh_result {
                MeshLoadResult::Quit => break,
                MeshLoadResult::Loaded(mesh) => {mesh_registry.add(mesh)}
            }
        }
    }
    child.join().unwrap();
}
