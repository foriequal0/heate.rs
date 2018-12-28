extern crate ctrlc;
extern crate num_cpus;
extern crate cgmath;
extern crate rand;

mod stop_handle;
mod heater;
mod cpu_heater;

use std::env::args;

use cpu_heater::CpuHeater;
use heater::Heater;

fn main() {
    let heater_count = args().nth(1)
        .map(|arg| str::parse::<usize>(&arg))
        .unwrap_or_else(|| Ok(num_cpus::get()))
        .expect("Argument error");
    let heater = CpuHeater::new(heater_count);

    let stop_handle = heater.get_stop_handle();
    ctrlc::set_handler(move || {
        stop_handle.stop();
    }).expect("Error setting Ctrl-C handler");

    println!("Turned on!");
    heater.start();
    println!("Turned off!");
}
