use regex::Regex;

fn main() {
    println!("Vad är ditt personnummer?");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    let re =
        Regex::new(r"^(19|20)?\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])[-+]?[0-9]{4}$").unwrap();

    if re.is_match(&input) {
        println!("Korrekt personnummer!");
    } else {
        println!("Ej korrekt personnummer.");
    }
}
