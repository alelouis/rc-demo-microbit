use egui::plot::{GridInput, GridMark, Legend, Line, LineStyle, Plot, Value, Values};
use egui::*;

use std::time::Duration;
use std::io;
use serialport::SerialPort;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    time: f64,
    #[serde(skip)]
    data_line: Vec<Value>,
    #[serde(skip)]
    port: Box<dyn SerialPort>,
    initialized: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "RC Charge and Discharge".to_owned(),
            value: 2.7,
            time: 0.0,
            data_line: vec![Value{x:0.0, y:0.0}; 512],
            port: serialport::new("/dev/cu.usbmodem1102", 115_200)
                .timeout(Duration::from_millis(10))
                .open().expect("Failed to open port"),
            initialized: false,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()

    }

    fn init_com(&mut self) {
        let mut serial_buf: Vec<u8> = vec![0; 128];
        match self.port.read(serial_buf.as_mut_slice()) {
            Ok(t) => println!("Initializing : read {} bytes", t),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        self.initialized = true;
    }

    fn get_data(&mut self) -> Line {
        if self.initialized == false {
            self.init_com();
        }

        let port = &mut self.port;
        let mut recover_buff: Vec<u8> = Vec::new();
        let data_line = &mut self.data_line;
        let mut serial_buf: Vec<u8> = vec![0; 128];

        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                for idx in 0..t {
                    recover_buff.push(serial_buf[idx]);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }

        while recover_buff.len() >= 2 {
            let data = [recover_buff[1], recover_buff[0]];
            let data_i16 = i16::from_be_bytes(data);
            recover_buff.drain(0..2);
            data_line.rotate_left(1);
            data_line[0] = Value{x:0.0, y:data_i16 as f64};
        }

        let mut idx = 0.0;
        for data_point in data_line.iter_mut() {
            data_point.x = idx;
            idx += 1.0;
        }

        Line::new(Values::from_values(data_line.to_vec()))
            .color(Color32::from_rgb(200, 100, 100))
            .style(LineStyle::Solid)
            .name("wave")
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value , time, port, data_line, initialized} = self;


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();
            self.time += ui.input().unstable_dt.at_most(1.0 / 30.0) as f64;
            let mut plot = Plot::new("lines_demo").legend(Legend::default());
            plot = plot.view_aspect(1.0);
            plot = plot.include_y(16384.0);
            plot = plot.include_y(0.0);
            plot.show(ui, |plot_ui| {
                plot_ui.line(self.get_data());
            });
        });


        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
