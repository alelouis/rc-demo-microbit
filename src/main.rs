#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui::{Style, Visuals};
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(rc_demo_microbit::TemplateApp::new(cc))
        }),
    );
}



/*
use std::time::Duration;
use std::io;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/cu.usbmodem1102", 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let mut recover_buff: Vec<u8> = Vec::new();
    let mut head: u32 = 0;
    loop {
        let mut serial_buf: Vec<u8> = vec![0; 128];
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                for idx in 0..t {
                    recover_buff.push(serial_buf[idx]);
                }
                //println!("{:?}", recover_buff);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }

        while recover_buff.len() > 2 {
            let data = [recover_buff[0], recover_buff[1]];
            let data_i16 = i16::from_be_bytes(data);
            recover_buff.drain(0..2);
            println!("{}", data_i16);
        }

    }

}

*/