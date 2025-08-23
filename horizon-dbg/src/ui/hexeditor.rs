use egui::{Align2, Color32, FontId, Pos2, Rect, Vec2, Widget};

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
    let font_id = FontId::monospace(20.0);

    let glyph_size: Vec2 = ui.fonts(|font| Vec2 {
      x: font.glyph_width(&font_id, 'a'),
      y: font.row_height(&font_id),
    });

    let byte_padding = glyph_size.x / 3.0;
    let max_col_groups = self.bytes_per_row / 4;
    let offset_width = glyph_size.x * 8.0 + byte_padding * 2.0;
    let num_rows = 1.0 + (self.data.len() as f32 / self.bytes_per_row as f32).ceil();

    let (widget_rect, response) = ui.allocate_exact_size(
      Vec2 {
        x: offset_width
          + self.bytes_per_row as f32 * glyph_size.x * 2.0
          + (self.bytes_per_row as f32 - 1.0) * byte_padding
          + max_col_groups as f32 * byte_padding,
        y: num_rows * (glyph_size.y + byte_padding),
      },
      egui::Sense::hover() | egui::Sense::click(),
    );

    let painter = ui.painter_at(widget_rect);

    ui.horizontal(|_ui| {
      // draw offsets at top
      for col in 0..16 {
        let col_group = col / 4;

        let x = widget_rect.left()
          + offset_width
          + col as f32 * (glyph_size.x * 2.0 + byte_padding)
          + col_group as f32 * byte_padding;
        let y = widget_rect.top();

        painter.text(
          Pos2 { x, y },
          Align2::LEFT_TOP,
          format!("{col:02X}"),
          font_id.clone(),
          Color32::GRAY,
        );
      }

      for (row, chunk) in self.data.chunks(self.bytes_per_row).enumerate() {
        let row_y = (row as f32 + 1.0) * (glyph_size.y + byte_padding);

        painter.text(
          Pos2 {
            x: widget_rect.left(),
            y: widget_rect.top() + row_y,
          },
          Align2::LEFT_TOP,
          format!("{:08X}", row * self.bytes_per_row),
          font_id.clone(),
          Color32::GRAY,
        );

        for (col, byte) in chunk.iter().enumerate() {
          let i = row * self.bytes_per_row + col;
          let col_group = col / 4;

          let x = widget_rect.left()
            + offset_width
            + col as f32 * (glyph_size.x * 2.0 + byte_padding)
            + col_group as f32 * byte_padding;
          let y = widget_rect.top() + row_y;

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
            format!("{:02X}", byte),
            font_id.clone(),
            Color32::WHITE,
          );

          if response.clicked() && hovered {
            *self.selected = Some(i);
          }
        }
      }
    });

    response
  }
}
