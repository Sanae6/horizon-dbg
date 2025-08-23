use eframe::{App, CreationContext};
use egui_dock::{DockArea, DockState, TabViewer};

enum Pane {
  ModuleList,
  MemoryMap,
  HexEditor,
}

struct PaneViewer;

impl TabViewer for PaneViewer {
  type Tab = Pane;

  fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
    match tab {
      Pane::ModuleList => "Module List".into(),
      Pane::MemoryMap => "Memory Map".into(),
      Pane::HexEditor => "Hex Editor".into(),
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
      Pane::HexEditor => {
        ui.heading("epic");
      }
    }
  }
}

pub struct DebuggerApp {
  dock_state: DockState<Pane>,
}

impl DebuggerApp {
  pub fn new(_cc: &CreationContext<'_>) -> DebuggerApp {
    DebuggerApp {
      dock_state: DockState::new(vec![Pane::MemoryMap, Pane::ModuleList, Pane::HexEditor]),
    }
  }
}

impl App for DebuggerApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    DockArea::new(&mut self.dock_state).show(ctx, &mut PaneViewer);
  }
}
