use regex::Regex;
use std::io::{self, Write};
use chrono::{NaiveDate, NaiveDateTime, Datelike, Utc};

struct ParsedId {
  year_str: String,
  month: u32,
  day: u32,
  separator: String,
}

fn parse_id(id: &str) -> Result<ParsedId, String> {
  let regex_id = Regex::new(
    r"^(\d{2}|\d{4})(\d{2})(\d{2})([-+])?(\d{4})$"
  ).map_err(|e| format!("Regex-fel: {}", e))?;

  let cap = regex_id.captures(id).ok_or("Ogiltigt format")?;

  Ok(ParsedId {
    year_str: cap.get(1).ok_or("Fel i årtal")?.as_str().to_string(),
    month: cap.get(2).ok_or("Fel i månad")?.as_str().parse()
      .map_err(|_| "Kunde inte tolka månad".to_string())?,
    day: cap.get(3).ok_or("Fel i dag")?.as_str().parse()
      .map_err(|_| "Kunde inte tolka dag".to_string())?,
    separator: cap.get(4).map(|m| m.as_str()).unwrap_or("").to_string(),
  })
}

fn calculate_full_year(year_str: &str, today: NaiveDateTime) -> Result<i32, String> {
  if year_str.len() == 4 {
    let year: i32 = year_str.parse().map_err(|_| "Kunde inte tolka årtal".to_string())?;
    if year > today.year() {
      return Err("Ogiltigt årtal i framtiden".to_string());
    }
    if year < 1900 {
      return Err("Ogiltigt årtal i dåtiden".to_string());
    }
    Ok(year)
  } else {
    let short: i32 = year_str.parse().map_err(|_| "Kunde inte tolka kort årtal".to_string())?;
    let current = today.year() % 100;
    Ok(if short <= current { 2000 + short } else { 1900 + short })
  }
}

fn validate_date(year: i32, month: u32, day: u32) -> Result<NaiveDate, String> {
  NaiveDate::from_ymd_opt(year, month, day)
    .ok_or("Ogiltigt datum".to_string())
}

fn validate_separator(birthdate: NaiveDate, separator: &str, today: NaiveDateTime) -> Result<(), String> {
  let age = today.year()
    - birthdate.year()
    - if today.ordinal() < birthdate.ordinal() { 1 } else { 0 };

  if age >= 100 && separator != "+" {
    return Err("Fel separator: '+' krävs för personer 100 år eller äldre".into());
  }
  if age < 100 && separator != "-" && separator != "" {
    return Err("Fel separator: '-' krävs för personer under 100 år".into());
  }

  Ok(())
}

fn validate_swedish_id_with_date(id: &str, today: NaiveDateTime) -> Result<(), String> {
  let mut parsed = parse_id(id)?;

  let year = calculate_full_year(&parsed.year_str, today)?;

  if parsed.day >= 61 {
    parsed.day -= 60;
  }

  let birthdate = validate_date(year, parsed.month, parsed.day)?;
  validate_separator(birthdate, &parsed.separator, today)?;

  let digits: String = id.chars().filter(|c| c.is_ascii_digit()).collect();
  if digits.len() < 10 {
    return Err("För få siffror för Luhn-kontroll".into());
  }

  let luhn_digits = &digits[digits.len() - 10..];
  if !luhn_check(luhn_digits) {
    return Err("Ogiltig kontrollsiffra (Luhn)".into());
  }

  Ok(())
}

fn luhn_check(s: &str) -> bool {
  let mut sum = 0;
  for (i, c) in s.chars().enumerate() {
    let mut n = match c.to_digit(10) {
      Some(d) => d,
      None => return false,
    };
    if i % 2 == 0 {
      n *= 2;
      if n > 9 { n -= 9; }
    }
    sum += n;
  }
  sum % 10 == 0
}

fn now() -> NaiveDateTime {
  Utc::now().naive_utc()
}

fn main() {
  loop {
    print!("Ange personnummer: ");
    if io::stdout().flush().is_err() {
      continue;
    }

    let mut id = String::new();
    if io::stdin().read_line(&mut id).is_err() {
      continue;
    }

    let id = id.trim();
    match validate_swedish_id_with_date(id, now()) {
      Ok(_) => {
        println!("{} är giltigt personnummer", id);
        break;
      }
      Err(e) => println!("{} är ogiltigt personnummer: {}", id, e),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::NaiveDate;

  fn mock_today() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2025, 12, 6).unwrap()
      .and_hms_opt(12, 0, 0).unwrap()
  }

  #[test]
  fn test_valid_personnummer() {
    assert!(validate_swedish_id_with_date("19870604-6714", mock_today()).is_ok());
  }

  #[test]
  fn test_invalid_date() {
    assert_eq!(
      validate_swedish_id_with_date("19870230-1234", mock_today()).unwrap_err(),
      "Ogiltigt datum"
    );
  }

  #[test]
  fn test_invalid_luhn() {
    assert_eq!(
      validate_swedish_id_with_date("19870604-6715", mock_today()).unwrap_err(),
      "Ogiltig kontrollsiffra (Luhn)"
    );
  }

  #[test]
  fn test_old_person_wrong_separator() {
    assert_eq!(
      validate_swedish_id_with_date("19231201-1234", mock_today()).unwrap_err(),
      "Fel separator: '+' krävs för personer 100 år eller äldre"
    );
    assert!(validate_swedish_id_with_date("19231201+1234", mock_today()).is_ok());
  }

  #[test]
  fn test_young_person_wrong_separator() {
    assert_eq!(
      validate_swedish_id_with_date("20000101+1234", mock_today()).unwrap_err(),
      "Fel separator: '-' krävs för personer under 100 år"
    );
    assert!(validate_swedish_id_with_date("20000101-1234", mock_today()).is_ok());
  }

  #[test]
  fn test_coordination_number() {
    assert!(validate_swedish_id_with_date("19870664-6714", mock_today()).is_ok());
  }

  #[test]
  fn test_short_year_2000s() {
    assert!(validate_swedish_id_with_date("250101-1232", mock_today()).is_ok());
  }

  #[test]
  fn test_short_year_1900s() {
    assert!(validate_swedish_id_with_date("870101-1235", mock_today()).is_ok());
  }

  #[test]
  fn test_too_few_digits() {
    assert_eq!(
      validate_swedish_id_with_date("870101-123", mock_today()).unwrap_err(),
      "För få siffror för Luhn-kontroll"
    );
  }

  #[test]
  fn test_non_digit_in_luhn() {
    assert_eq!(
      validate_swedish_id_with_date("19870604-67A4", mock_today()).unwrap_err(),
      "Ogiltig kontrollsiffra (Luhn)"
    );
  }

  #[test]
  fn test_future_year_invalid() {
    assert_eq!(
      validate_swedish_id_with_date("20870604-6714", mock_today()).unwrap_err(),
      "Ogiltigt årtal i framtiden"
    );
  }
}
