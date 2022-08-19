mod fronius_getter;
mod utils;

const URL: &'static str = "http://192.168.1.100/solar_api/v1/GetPowerFlowRealtimeData.fcgi";

fn main() {
  let content = fronius_getter::web_get(URL);
  let powers = fronius_getter::retrieve_power_measure(&content);
  println!("Solar power : {:.0} W", powers.p_pv.abs());
  println!("Load power : {:.0} W", powers.p_load.abs());
  println!("Balance to grid : {} ", utils::format_grid_power(powers.p_grid));
}
