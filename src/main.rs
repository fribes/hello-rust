use curl::easy::Easy;

fn main() {
  let data = web_get("http://192.168.1.100/solar_api/v1/GetPowerFlowRealtimeData.fcgi");
  let power = parse_answer(data);
  println!("Power consumption {:.0} W", power);
}

fn web_get(url: &str) -> Vec<u8> {
  let mut data = Vec::new();
  let mut handle = Easy::new();
  handle.url(url).unwrap();
  {
      let mut transfer = handle.transfer();
      transfer.write_function(|new_data| {
          data.extend_from_slice(new_data);
          Ok(new_data.len())
      }).unwrap();
      transfer.perform().unwrap();
  }
  return data;
}

fn parse_answer (data: Vec<u8>) -> f64 {
  let body = std::str::from_utf8(&data).unwrap_or_else(|e| {
      panic!("Failed to get body; error is {}", e);
  });

  let json = json::parse(body).unwrap_or_else(|e| {
      panic!("Failed to parse json from {}; error is {}", body, e);
  });
  let p_grid = json["Body"]["Data"]["Site"]["P_Grid"].as_number();
  let raw = p_grid.unwrap_or_else(||{
      panic!("Failed to parse number");
  });
  let power: f64 = raw.into();

  return power;
}