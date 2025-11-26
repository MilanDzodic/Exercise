use regex::Regex;

fn main() {
    println!("Vad är ditt personnummer?");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    let re =
        Regex::new(r"^(19|20)?\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])[-+]?[0-9]{4}$").unwrap();

    if re.is_match(&input) {
        input = format_personnummer(&input).to_string();
        if luhn_check(&input) {
            println!("Korrekt personnummer!");
        }
    } else {
        println!("Ej korrekt personnummer.");
    }
}

fn luhn_check(personnummer: &str) -> bool {
    /*let mut sum = 0;*/
    println!("Testing luhn on: {}", personnummer);

    return true;
}

fn format_personnummer(personnummer: &str) -> String {
    let formatted_personnummer: String = personnummer.chars().filter(|c| c.is_digit(10)).collect();

    let clean_format = if formatted_personnummer.len() > 10 {
        &formatted_personnummer[2..]
    } else {
        &formatted_personnummer
    };

    println!("clean personnummer: {}", clean_format);

    return clean_format.to_string();
}

/* 19970222-9222 */
