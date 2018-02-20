use std::fs::File;
use std::io::Read;
use std::{thread, time};


// Raspberry Pi makes CPU temperature available in this file path:
const CPU_TEMPERATURE_PATH : &'static str =  "/sys/class/thermal/thermal_zone0/temp";

/// Read the RPi's CPU temperature and write it to stdout
///
fn read_and_display_temperature() {
  let mut f = File::open(CPU_TEMPERATURE_PATH)
    .expect("Could not open CPU_TEMPERATURE_PATH");

  let mut temp_str = String::new();
  f.read_to_string(&mut temp_str)
    .expect("Could not read temperature");

  let raw_temp : u32 = temp_str.trim().parse()
    .expect("Bogus raw temperature string");

  let temp_float : f32 = {raw_temp as f32} / 1000.0;
  println!("{}",temp_float);
}

fn main() {

  let read_delay_secs = time::Duration::from_secs(1);

  loop {
    read_and_display_temperature();
    thread::sleep(read_delay_secs);
  }
}
