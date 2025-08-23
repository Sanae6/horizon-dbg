mod hexeditor;

use eframe::{App, CreationContext};
use egui_dock::{DockArea, DockState, TabViewer};
use hexeditor::HexEditor;

enum Pane {
  ModuleList,
  MemoryMap,
  HexEditor {
    data: Vec<u8>,
    selected: Option<usize>,
  },
}

struct PaneViewer;

impl TabViewer for PaneViewer {
  type Tab = Pane;

  fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
    match tab {
      Pane::ModuleList => "Module List".into(),
      Pane::MemoryMap => "Memory Map".into(),
      Pane::HexEditor { .. } => "Hex Editor".into(),
    }
  }

  fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
    match tab {
      Pane::ModuleList => {
        ui.heading("awesome");
      }
      Pane::MemoryMap => {
        ui.heading("so cool");
      }
      Pane::HexEditor { data, selected } => {
        ui.add(HexEditor::new(data, selected));
      }
    }
  }
}

pub struct DebuggerApp {
  dock_state: DockState<Pane>,
}

impl DebuggerApp {
  pub fn new(cc: &CreationContext<'_>) -> DebuggerApp {
    replace_fonts(&cc.egui_ctx);

    DebuggerApp {
      dock_state: DockState::new(vec![
        Pane::MemoryMap,
        Pane::ModuleList,
        Pane::HexEditor {
          data: vec![0u8; 0x80],
          selected: None,
        },
      ]),
    }
  }
}

impl App for DebuggerApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // ctx.set_pixels_per_point(1.0);
    DockArea::new(&mut self.dock_state).show(ctx, &mut PaneViewer);
  }
}

fn replace_fonts(ctx: &egui::Context) {
  let mut fonts = egui::FontDefinitions::default();

  fonts.font_data.insert(
    "SF Mono".to_owned(),
    std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
      "SFMonoSquare-Regular.otf"
    ))),
  );

  fonts
    .families
    .entry(egui::FontFamily::Monospace)
    .or_default()
    .insert(0, "SF Mono".to_owned());

  ctx.set_fonts(fonts);
}
