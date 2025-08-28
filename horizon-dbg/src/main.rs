mod engine;
pub mod ui;
mod persistent;

use std::future::pending;

use tokio::runtime::Runtime;

use crate::ui::DebuggerApp;

fn main() {
  let rt = Runtime::new().expect("failed to initialize tokio runtime");
  let _guard = rt.enter();
  std::thread::spawn(move || {
    rt.block_on(pending::<()>());
  });
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "horizon-dbg",
    native_options,
    Box::new(|cc| Ok(Box::new(DebuggerApp::new(cc)))),
  )
  .unwrap();
}
