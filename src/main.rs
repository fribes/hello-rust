mod fronius_getter;

const URL: &'static str = "http://192.168.1.100/solar_api/v1/GetPowerFlowRealtimeData.fcgi";

fn main() {
  let content = fronius_getter::web_get(URL);
  let powers = fronius_getter::retrieve_power_measure(&content);
  println!("Solar power : {:.0} W", powers.p_pv.abs());
  println!("Load power : {:.0} W", powers.p_load.abs());
  println!("Balance to grid : {} ", format_grid_power(powers.p_grid));
}

fn format_grid_power( power: f32) -> String {
  let mut direction = "";
  if power < 0.0 {
    direction = "injecting";
  }
  else if power > 0.0 {
    direction = "drawing"
  }
  return format!("{} {:.0} W", direction, power.abs())
}
