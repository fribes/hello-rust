mod fronius;
mod utils;

fn main() {
  let content = fronius::get();
  let powers = fronius::retrieve_power_measure(&content);
  println!("Solar power : {:.0} W", powers.p_pv.abs());
  println!("Load power : {:.0} W", powers.p_load.abs());
  println!("Balance to grid : {} ", utils::format_grid_power(powers.p_grid));
}
