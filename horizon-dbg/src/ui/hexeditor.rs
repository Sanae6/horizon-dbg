use egui::{Align2, Color32, FontId, Pos2, Rect, TextStyle, Vec2, Widget};

pub struct HexEditor<'a> {
  data: &'a mut [u8],
  bytes_per_row: usize,
  selected: &'a mut Option<usize>,
}

impl<'a> HexEditor<'a> {
  pub fn new(data: &'a mut [u8], selected: &'a mut Option<usize>) -> Self {
    Self {
      data,
      bytes_per_row: 16,
      selected,
    }
  }
}

impl<'a> Widget for HexEditor<'a> {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    let font_id = egui::FontId::monospace(20.0);

    let glyph_size: Vec2 = ui.fonts(|font| Vec2 {
      x: font.glyph_width(&font_id, 'a'),
      y: font.row_height(&font_id),
    });

    let byte_padding = glyph_size.x / 3.0;

    let (widget_rect, response) = ui.allocate_exact_size(
      Vec2 {
        x: self.bytes_per_row as f32 * (glyph_size.x * 2.0 + byte_padding) - byte_padding,
        y: 10.0 * glyph_size.y,
      },
      egui::Sense::hover() | egui::Sense::click(),
    );

    let painter = ui.painter_at(widget_rect);

    // draw background
    // painter.rect_filled(outer_rect, 0.0, Color32::DARK_GRAY);

    ui.horizontal(|ui| {
      for (i, byte) in self.data.iter().enumerate() {
        let row = i / self.bytes_per_row;
        let col = i % self.bytes_per_row;

        let x = widget_rect.left() + col as f32 * (glyph_size.x * 2.0 + byte_padding);
        let y = widget_rect.top() + row as f32 * (glyph_size.y + byte_padding);

        let outer_rect = Rect::from_min_size(
          Pos2 {
            x: x - byte_padding / 2.0,
            y: y - byte_padding / 2.0,
          },
          Vec2 {
            x: glyph_size.x * 2.0 + byte_padding,
            y: glyph_size.y + byte_padding,
          },
        );

        let hovered = response
          .hover_pos()
          .map(|p| outer_rect.contains(p))
          .unwrap_or(false);

        if *self.selected == Some(i) {
          painter.rect_filled(outer_rect, 0.0, Color32::GRAY);
        } else if hovered {
          painter.rect_filled(outer_rect, 0.0, Color32::DARK_GRAY);
        }

        painter.text(
          Pos2 { x, y },
          Align2::LEFT_TOP,
          format!("{:02x}", byte),
          font_id.clone(),
          Color32::WHITE,
        );

        if response.clicked() && hovered {
          *self.selected = Some(i);
        }
      }
    });

    response
  }
}
