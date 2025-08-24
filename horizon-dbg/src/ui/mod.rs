mod hexeditor;

use eframe::{App, CreationContext};
use egui::{MenuBar, ScrollArea, TopBottomPanel, scroll_area::ScrollSource};
use egui_dock::{DockArea, DockState, TabViewer};

use crate::ui::hexeditor::{HexEditor, HexEditorState};

enum Pane {
  ModuleList,
  MemoryMap,
  InstructionListing,
  ProcessList,
  StructViewer,

  HexEditor {
    data: Vec<u8>,
    selected: Option<usize>,
    state: HexEditorState,
  },
}

struct PaneViewer;

impl TabViewer for PaneViewer {
  type Tab = Pane;

  fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
    match tab {
      Pane::ModuleList => "Module List".into(),
      Pane::MemoryMap => "Memory Map".into(),
      Pane::InstructionListing => "Listing".into(),
      Pane::ProcessList => "Processes".into(),
      Pane::StructViewer => "Struct Viewer".into(),
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
      Pane::HexEditor {
        data,
        selected,
        state,
      } => {
        ScrollArea::vertical()
          .scroll_source(ScrollSource::MOUSE_WHEEL | ScrollSource::SCROLL_BAR)
          .show(ui, |ui| {
            ui.add(HexEditor::new(data, selected, state));
          });
      }
      _ => {}
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
          data: vec![0u8; 0x1000],
          selected: None,
          state: HexEditorState::Idle,
        },
      ]),
    }
  }
}

impl App for DebuggerApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // ctx.set_pixels_per_point(1.0);
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
      MenuBar::new().ui(ui, |ui| {
        ui.menu_button("Windows", |ui| {
          if ui.button("Hex Editor").clicked() {
            self.dock_state.add_window(vec![Pane::HexEditor {
              data: vec![0u8; 0x1000],
              selected: Some(0),
              state: HexEditorState::Idle,
            }]);
          }
        })
      });
    });

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
