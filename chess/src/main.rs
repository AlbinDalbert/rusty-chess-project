use std::io;

use eframe::{run_native, epi::App, egui::CentralPanel, NativeOptions};


struct ChessGame;

impl App for ChessGame {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel:: default().show(ctx, |ui|{
            ui.label("Board");
        });

        
    }

    fn name(&self) -> &str {
        "Chess Game"
    }
}


fn main() {
    println!("Hello, chess!");
    
    let app = ChessGame;
    let win_option = NativeOptions::default();

    run_native(Box::new(app), win_option);

    
}