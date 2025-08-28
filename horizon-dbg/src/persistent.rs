use std::{
  ops::{Deref, DerefMut},
  path::PathBuf,
  sync::{LazyLock, MutexGuard},
};

use savefile::prelude::*;
use std::sync::Mutex;

const VERSION: u32 = 0;

#[derive(Default, Savefile)]
pub struct State {
  pub last_host: String,
  pub last_vendor_id: u16,
  pub last_product_id: u16,
  pub saved_targets: Vec<Target>,
}

#[derive(PartialEq, Eq, Clone, Savefile)]
pub enum Target {
  Tcp { host: String },
  Usb { vendor_id: u16, product_id: u16 },
}

impl Default for Target {
  fn default() -> Self {
    Self::Tcp {
      host: String::new(),
    }
  }
}

static PATH: LazyLock<PathBuf> = LazyLock::new(|| {
  let base = dirs::data_dir()
    .or(dirs::home_dir())
    .unwrap_or_else(|| PathBuf::from("./"));

  base.join("HorizonDbg.bin")
});
pub static PERSISTENT_STATE: LazyLock<StateHolder> = LazyLock::new(|| {
  let state = match load_file::<State, &PathBuf>(&*PATH, VERSION) {
    Ok(state) => state,
    Err(error) => {
      eprintln!("failed to load save file: {error:#?}");
      State::default()
    }
  };

  StateHolder {
    state: Mutex::new(state),
  }
});
pub struct StateHolder {
  state: Mutex<State>,
}

#[allow(unused)]
impl StateHolder {
  pub fn get(&'static self) -> StateGuard<'static> {
    StateGuard(self.state.lock().expect("poisoned state"))
  }
  pub fn get_mut(&'static self) -> StateGuardMut<'static> {
    StateGuardMut(self.state.lock().expect("poisoned state"))
  }
}

pub struct StateGuard<'a>(MutexGuard<'a, State>);
impl<'a> Deref for StateGuard<'a> {
  type Target = State;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

pub struct StateGuardMut<'a>(MutexGuard<'a, State>);

impl<'a> Deref for StateGuardMut<'a> {
  type Target = State;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl<'a> DerefMut for StateGuardMut<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0.deref_mut()
  }
}

impl<'a> Drop for StateGuard<'a> {
  fn drop(&mut self) {
    if let Err(error) = save_file(&*PATH, VERSION, &*self.0) {
      eprintln!("failed to save state: {error:#?}");
    }
  }
}
