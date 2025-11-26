use regex::Regex;

fn main() {
    println!("Vad är ditt personnummer?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    /* Regex pattern for Swedish Personnummer */
    let re =
        Regex::new(r"^(19|20)?\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])[-+]?[0-9]{4}$").unwrap();

    /* Validate - Format - Luhn check */
    if re.is_match(&input) {
        input = format_personnummer(&input);
        if luhn_check(&input) {
            println!("Korrekt personnummer!");
        } else {
            println!("Ej korrekt personnummer.");
        }
    } else {
        println!("Ej korrekt personnummer.");
    }
}

fn luhn_check(personnummer: &str) -> bool {
    let mut sum = 0;

    for (i, c) in personnummer.chars().enumerate() {
        /* In the loop, if the current index is even multipy the number by 2 */
        let mut num = c.to_digit(10).unwrap();

        if i % 2 == 0 {
            num *= 2;
        }

        /* If value of num is greater than 9, we need to subract it by 9 for the algorithm */
        if num > 9 {
            num -= 9;
        };

        /* Add the value of num to sum */
        sum += num;
    }

    println!("sum = {}, bool = {}", sum, sum % 10 == 0);

    /* Return boolean of sum modulus 10 */
    return sum % 10 == 0;
}

fn format_personnummer(personnummer: &str) -> String {
    /* Filter to remove hyphens & the keep numbers */
    let formatted_personnummer: String = personnummer.chars().filter(|c| c.is_digit(10)).collect();

    /* Ensure 10 digit format for luhn_check algorithm, slice if needed */
    let clean_format = if formatted_personnummer.len() > 10 {
        &formatted_personnummer[2..]
    } else {
        &formatted_personnummer
    };

    println!("clean personnummer: {}", clean_format);

    /* Return String (Rust Ownership) */
    return clean_format.to_string();
}

/* Test
19970222-9222
199702229222

970222-9222
9702229222

19811218-9876
198112189876

811218-9876
8112189876
*/
