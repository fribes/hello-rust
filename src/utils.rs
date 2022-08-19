pub fn format_grid_power(power: f32) -> String {
  let mut direction = "";
  if power < 0.0 {
    direction = "injecting";
  }
  else if power > 0.0 {
    direction = "drawing"
  }
  return format!("{} {:.0} W", direction, power.abs())
}

#[test]
fn test_format_grid_power() {
    assert_eq!(format_grid_power(0.0), " 0 W");
}

#[test]
fn test_format_grid_power_injecting() {
    assert_eq!(format_grid_power(-1450.0), "injecting 1450 W");
}

#[test]
fn test_format_grid_power_drawing() {
    assert_eq!(format_grid_power(550.0), "drawing 550 W");
}

pub fn format_generic_power(power: f32) -> String {
  return format!("{:.0} W", power.abs())
}

#[test]
fn test_format_generic_power_pos() {
    assert_eq!(format_generic_power(550.0), "550 W");
}

#[test]
fn test_format_generic_power_neg() {
    assert_eq!(format_generic_power(-432.0), "432 W");
}
