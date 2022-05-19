use crate::CommBridge;

pub struct TemplateApp {
    comm_bridge: CommBridge,
    textbuf: String,
    js_log: Vec<String>,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, bridge: CommBridge) -> Self {
        Self {
            comm_bridge: bridge,
            textbuf: String::new(),
            js_log: Vec::new(),
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Every update, check for messages from js
        while let Some(msg) = self.comm_bridge.pull_from_js() {
            self.js_log.push(msg);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Your message");
            if ui.text_edit_singleline(&mut self.textbuf).lost_focus() {
                // Send a message to js
                self.comm_bridge.push_to_js(&self.textbuf);
                self.textbuf.clear();
            }
            ui.separator();
            for entry in &self.js_log {
                ui.label(format!("Js: {}", entry));
            }
        });
    }
}
