use eframe::egui::{Color32, Rounding, Ui};

pub const DARKER: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 64);

pub fn set_input_rounding(ui: &mut Ui) {
  const INPUT_ROUNDING: Rounding = Rounding::same(8.0);

  let widgets = &mut ui.visuals_mut().widgets;
  widgets.inactive.rounding = INPUT_ROUNDING;
  widgets.active.rounding = INPUT_ROUNDING;
  widgets.hovered.rounding = INPUT_ROUNDING;
}