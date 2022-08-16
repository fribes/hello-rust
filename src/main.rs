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

fn parse_answer (body: &str) -> f32 {
  let json = json::parse(body).unwrap_or_else(|e| {
      panic!("Failed to parse json; error is {}", e);
  });
  let p_grid = json["Body"]["Data"]["Site"]["P_Grid"].as_number();
  let raw = p_grid.unwrap_or_else(||{
      panic!("Failed to parse number");
  });
  let power: f32 = raw.into();

  power
}

#[test]
fn test_parse_answer() {
    assert_eq!(parse_answer("{ \"Body\": { \"Data\": { \"Site\": { \"P_Grid\": 234 }}} }"), 234 as f32);
}

#[test]
#[should_panic(expected = "Failed to parse json; error is Unexpected end of JSON")]
fn test_parse_bad_json() {
  parse_answer("");
}

#[test]
#[should_panic(expected = "Failed to parse number")]
fn test_parse_missing_attribute() {
  parse_answer("{ \"Body\": { \"Data\": { \"Site\": {} }} }");
}

#[test]
#[should_panic(expected = "Failed to parse number")]
fn test_parse_not_a_number() {
  parse_answer("{ \"Body\": { \"Data\": { \"Site\": { \"P_Grid\": \"not a number\" }}} }");
}

