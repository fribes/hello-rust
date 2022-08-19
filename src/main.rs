mod fronius;
mod utils;

fn main() {
  let content = fronius::get();
  let powers = fronius::retrieve_power_measure(&content);
  println!("Solar power : {}", utils::format_generic_power(powers.p_pv));
  println!("Load power : {}", utils::format_generic_power(powers.p_load));
  println!("Balance to grid : {}", utils::format_grid_power(powers.p_grid));
}
