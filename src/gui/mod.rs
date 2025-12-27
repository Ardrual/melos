mod piano_roll;

use std::path::PathBuf;
use anyhow::Result;
use eframe::egui;

use crate::ast::Score;
use crate::ir::IrScore;

pub use piano_roll::PianoRollApp;

pub fn run_gui(score_path: PathBuf, ast: Score, ir: IrScore) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 700.0])
            .with_min_inner_size([800.0, 400.0]),
        ..Default::default()
    };

    let title = ast.headers.iter()
        .find_map(|h| {
            if let crate::ast::Header::Title(t) = h {
                Some(t.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "Melos".to_string());

    eframe::run_native(
        &format!("Melos - {}", title),
        options,
        Box::new(|cc| {
            Ok(Box::new(PianoRollApp::new(cc, score_path, ast, ir)))
        }),
    ).map_err(|e| anyhow::anyhow!("Failed to run GUI: {}", e))
}
