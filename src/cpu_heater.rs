use std::thread::spawn;

use super::stop_handle::StopHandle;
use heater::Heater;
use cgmath::Matrix4;
use rand::{self, Rng};

pub struct CpuHeater {
    threads: usize,
    stop_handle: StopHandle,
}

impl CpuHeater {
    pub fn new(threads: usize) -> CpuHeater {
        CpuHeater { threads, stop_handle: StopHandle::new() }
    }
}

fn work(stop_handle: &StopHandle) {
    let mut v = Vec::new();
    for _ in 0..1000000 {
        v.push(Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ));
    }
    let mut rng = rand::thread_rng();
    while !stop_handle.stopped() {
        let (a, b, c) = (rng.gen_range(0, 16), rng.gen_range(0, 16), rng.gen_range(0, 16));
        v[a] = v[b] * v[c];
    }
}

impl Heater for CpuHeater {
    fn get_stop_handle(&self) -> StopHandle {
        self.stop_handle.clone()
    }

    fn start(&self) {
        println!("{} cpus are used as heater", self.threads);
        let threads: Vec<_> = (0..self.threads)
            .map(|_| {
                let stop_handle = self.stop_handle.clone();
                spawn(move ||work(&stop_handle))
            }).collect();

        for th in threads.into_iter() {
            th.join().unwrap();
        }
    }
}
