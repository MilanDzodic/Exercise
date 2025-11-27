use regex::Regex;
use std::io::{self, Write};
use chrono::{NaiveDate, Datelike, Utc};

fn validate_swedish_id(id: &str) -> Result<(), String> {
  let re = Regex::new(
    r"^(?:(?P<full>(?:19|20)\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01]|6[1-9]|[7-8]\d|9[0-1]))|(?P<short>\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01]|6[1-9]|[7-8]\d|9[0-1])))(?P<sep>[-+])?(?P<last>\d{4})$"
  ).unwrap();

  let caps = re.captures(id).ok_or("Ogiltigt format")?;

  // Hämta sep
  let separator = caps.name("sep").map(|m| m.as_str()).unwrap_or("");

  // Plocka ut delar beroende på full/short format
  let (year, month, mut day) = if let Some(full) = caps.name("full") {
    let full = full.as_str();
    (
      full[0..4].parse().unwrap(),
      full[4..6].parse().unwrap(),
      full[6..8].parse().unwrap(),
    )
  } else {
    let short = caps.name("short").unwrap().as_str();
    let yy: i32 = short[0..2].parse().unwrap();
    let mm: u32 = short[2..4].parse().unwrap();
    let dd: u32 = short[4..6].parse().unwrap();

    let current_year = Utc::now().year() % 100;
    let full_year = if yy <= current_year { 2000 + yy } else { 1900 + yy };

    (full_year, mm, dd)
  };

  // Samordningsnummer (dag 61–91)
  let is_coord = day >= 61;
  if is_coord {
    day -= 60;
  }

  // Datumkontroll
  let birthdate = NaiveDate::from_ymd_opt(year, month, day)
    .ok_or("Ogiltigt datum")?;

  // Separator-kontroll
  let today = Utc::now().naive_utc();
  let age = today.year()
    - birthdate.year()
    - if today.ordinal() < birthdate.ordinal() { 1 } else { 0 };

  if age >= 100 && separator != "+" {
    return Err("Fel separator: '+' krävs för personer 100 år eller äldre".to_string());
  }
  if age < 100 && separator != "-" && separator != "" {
    return Err("Fel separator: '-' krävs för personer under 100 år".to_string());
  }

  // Luhn
  let digits: String = id.chars().filter(|c| c.is_ascii_digit()).collect();
  let luhn_digits = &digits[digits.len() - 10..];

  if !luhn_check(luhn_digits) {
    return Err("Ogiltig kontrollsiffra (Luhn)".to_string());
  }

  Ok(())
}

fn luhn_check(s: &str) -> bool {
  let mut sum = 0;
  for (i, c) in s.chars().enumerate() {
    let mut n = c.to_digit(10).unwrap();
    if i % 2 == 0 {
      n *= 2;
      if n > 9 { n -= 9; }
    }
    sum += n;
  }
  sum % 10 == 0
}

fn main() {
  loop {
  print!("Ange personnummer att testa: ");
    io::stdout().flush().unwrap();

    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Fel vid läsning av input");  

    let id = id.trim();

  // let examples = [
  //   "19900229-1234",
  //   "20010631+4321",
  //   "19100101-1237",
  //   "19870604+6714",
  //   "19870604-6714",
  //   "198706046714",
  //   "870604-6714",
  //   "8706046714",
  //   "8706046715",
  //   "20006101-1234",
  //   "19206101+1234",
  //   "990229-1231",
  //   "19206101-1234",
  // ];

  // for id in examples {
    match validate_swedish_id(id) {
      Ok(_) => {
        println!("{} är giltigt personnummer", id);
        break;
      }
      Err(e) =>
      println!("{} är ogiltigt personnummer: {}", id, e)
    }
  // }  
  }
}
