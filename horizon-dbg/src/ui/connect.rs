use std::{
  io,
  net::{IpAddr, SocketAddr},
};

use eframe::CreationContext;
use egui::{Area, DragValue, Grid, Layout, Ui, UiBuilder};
use nusb::{DeviceInfo, MaybeFuture};
use tokio::sync::oneshot;

use crate::{
  engine::{
    DebuggerInterface,
    connection::{self, Connection},
  },
  persistent::{PERSISTENT_STATE, Target},
};

pub struct ConnectMenu {
  connecting: Option<oneshot::Receiver<io::Result<Connection>>>,
  address_text: String,
  error: Option<io::Error>,
  usb_devices: Result<Vec<DeviceInfo>, nusb::Error>,
  selected_device: usize,
  saved_targets: Vec<Target>,
}

pub enum NextScreen {
  Current,
  Debugger(DebuggerInterface),
}

impl ConnectMenu {
  pub fn new() -> Self {
    let state = PERSISTENT_STATE.get();

    // todo: make this a soft error
    let usb_devices = MaybeFuture::wait(nusb::list_devices()).map(|iter| iter.collect::<Vec<_>>());

    let selected_device = usb_devices
      .as_ref()
      .map(|devices| {
        devices
          .iter()
          .enumerate()
          .find(|(_, device)| {
            device.vendor_id() == state.last_vendor_id
              && device.product_id() == state.last_product_id
          })
          .map(|(index, _)| index)
          .unwrap_or(0)
      })
      .unwrap_or(0);

    Self {
      connecting: None,
      address_text: state.last_host.clone(),
      error: None,
      usb_devices,
      selected_device,
      saved_targets: state.saved_targets.clone(),
    }
  }

  pub fn show(&mut self, ui: &mut Ui) -> NextScreen {
    ui.set_min_width(300.0);
    ui.set_min_height(100.0);
    ui.heading("Connect");
    ui.separator();
    ui.horizontal(|ui| {
      ui.vertical(|ui| {
        ui.label("USB");
        Grid::new("device_grid").striped(true).show(ui, |ui| {
          if self.connecting.is_some() {
            ui.disable();
          }
          for (index, device) in self.usb_devices.iter().flatten().enumerate() {
            if ui
              .radio_value(
                &mut self.selected_device,
                index,
                format!(
                  "{} ({:04X}:{:04X})",
                  device.product_string().unwrap_or("Unknown Device"),
                  device.vendor_id(),
                  device.product_id()
                ),
              )
              .clicked()
            {};
            ui.end_row();
          }
        });

        if self.connecting.is_some() || true {
          ui.disable();
        }
        if ui.button("Connect").clicked() {
          todo!()
        }
      });
      ui.separator();
      ui.vertical(|ui| {
        ui.label("TCP");
        ui.horizontal(|ui| {
          ui.label("Host");
          if self.connecting.is_some() {
            ui.disable();
          }
          if ui.text_edit_singleline(&mut self.address_text).changed() {
            PERSISTENT_STATE.get_mut().last_host = self.address_text.clone();
          }
        });

        // todo: cache and update on address string change
        let socket_addr = self
          .address_text
          .as_str()
          .parse::<SocketAddr>()
          .or_else(|_| {
            self
              .address_text
              .as_str()
              .parse::<IpAddr>()
              .map(|ip| SocketAddr::new(ip, 2434))
          });

        if socket_addr.is_err() || self.connecting.is_some() {
          ui.disable();
        }

        ui.label(format!("{socket_addr:?}"));

        if ui.button("Connect").clicked()
          && let Ok(socket_addr) = socket_addr
        {
          self.connecting = Some(Connection::connect_tcp(socket_addr));
        }
      });
    });

    if let Some(receiver) = self
      .connecting
      .take_if(|receiver| !receiver.is_empty() || receiver.is_terminated())
    {
      match receiver.blocking_recv() {
        Ok(Ok(connection)) => NextScreen::Debugger(DebuggerInterface::new(connection)),
        Ok(Err(error)) => {
          self.error = Some(error);
          NextScreen::Current
        }
        Err(_) => {
          panic!("connection task panicked");
        }
      }
    } else {
      NextScreen::Current
    }
  }
}
