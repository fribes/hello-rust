use curl::easy::Easy;

const URL: &'static str = "http://192.168.1.100/solar_api/v1/GetPowerFlowRealtimeData.fcgi";

struct Power {
    p_grid: f32,
    p_load: f32,
    p_pv: f32
}

fn main() {
  let content = web_get(URL);
  let powers = retrieve_power_measure(&content);
  println!("Grid power : {} ", format_grid_power(powers.p_grid));
  println!("Load power : {:.0} W", powers.p_load.abs());
  println!("PV power : {:.0} W", powers.p_pv.abs());
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


fn web_get(url: &str) -> String {
  let mut handle = Easy::new();
  let mut data = Vec::new();
  handle.url(url).unwrap();
  {
      let mut transfer = handle.transfer();
      transfer.write_function(|new_data| {
          data.extend_from_slice(new_data);
          Ok(new_data.len())
      }).unwrap();
      transfer.perform().unwrap();
  }
  let body = std::str::from_utf8(&data).unwrap_or_else(|e| {
      panic!("Failed to get body; error is {}", e);
  });

  (*body).to_string()
}

fn retrieve_power_measure (body: &str) -> Power {
  let json = json::parse(body).unwrap_or_else(|e| {
      panic!("Failed to parse json; error is {}", e);
  });

  let p_grid = json["Body"]["Data"]["Site"]["P_Grid"].as_number();
  let p_load = json["Body"]["Data"]["Site"]["P_Load"].as_number();
  let p_pv = json["Body"]["Data"]["Site"]["P_PV"].as_number();

  return Power {
    p_grid: json_to_f32(p_grid),
    p_load: json_to_f32(p_load),
    p_pv: json_to_f32(p_pv)
  };
}

fn json_to_f32(json_value: Option<json::number::Number>) -> f32 {
  return json_value.unwrap_or_else(||{
        panic!("Failed to parse number")}).into()
}

#[test]
fn test_retrieve_power_measure() {
    assert_eq!(retrieve_power_measure("{ \"Body\": { \"Data\": { \"Site\": { \"P_Grid\": 234, \"P_Load\": 123, \"P_PV\": 22 }}} }").p_grid, 234 as f32);
}

#[test]
#[should_panic(expected = "Failed to parse json; error is Unexpected end of JSON")]
fn test_parse_bad_json() {
  retrieve_power_measure("");
}

#[test]
#[should_panic(expected = "Failed to parse number")]
fn test_parse_missing_attribute() {
  retrieve_power_measure("{ \"Body\": { \"Data\": { \"Site\": {} }} }");
}

#[test]
#[should_panic(expected = "Failed to parse number")]
fn test_parse_not_a_number() {
  retrieve_power_measure("{ \"Body\": { \"Data\": { \"Site\": { \"P_Grid\": \"not a number\" }}} }");
}

