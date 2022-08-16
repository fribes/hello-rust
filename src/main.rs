use curl::easy::Easy;

const URL: &'static str = "http://192.168.1.100/solar_api/v1/GetPowerFlowRealtimeData.fcgi";

fn main() {
  let content = web_get(URL);
  let power = parse_answer(&content);
  println!("Power consumption {:.0} W", power);
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

fn parse_answer (body: &str) -> f64 {
  let json = json::parse(body).unwrap_or_else(|e| {
      panic!("Failed to parse json from {}; error is {}", body, e);
  });
  let p_grid = json["Body"]["Data"]["Site"]["P_Grid"].as_number();
  let raw = p_grid.unwrap_or_else(||{
      panic!("Failed to parse number");
  });
  let power: f64 = raw.into();

  power
}
