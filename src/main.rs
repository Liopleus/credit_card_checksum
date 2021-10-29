use std::io;

fn main() {
    enum Card {
        VISA,
        MasterCard,
        AMEX,
        INVALID,
    }

    loop {
        // Display program description
        println!("This program takes a credit card number and check if it's a valid credit card issued by VISA, MasterCard or American Express.");
        println!("Please enter the card number:");

        // Initialize card status
        let mut stat = Card::INVALID;

        // Takes user input
        let card_number: u64;
        let card_number_len: usize;
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Invalid card number!");
    
            // Remove space and hyphen from the input
            input = input.replace(&[' ','-'][..],"");
    
            // Convert input into a number
            match input.trim().parse() {
                Ok(num) => {
                    card_number = num;
                    card_number_len = input.trim().len();
                    break
                }
                _ => println!("Please enter a valid card number!")
            }
        }

        // Identify card issuer and validate the card with checksum
        match card_number_len {
            13 => {
                match card_number / 10_u64.pow(card_number_len as u32 - 1) { // Extract the first digit
                    4 => {
                        match checksum(card_number, card_number_len) {
                            true => stat = Card::VISA,
                            false => ()
                        }
                    },
                    _ => ()
                }
            },
            15 => {
                match card_number / 10_u64.pow(card_number_len as u32 - 2) {
                    34 | 37 => {
                        match checksum(card_number, card_number_len) {
                            true => stat = Card::AMEX,
                            false => ()
                        }
                    },
                    _ => ()
                }
            },
            16 => {
                match card_number / 10_u64.pow(card_number_len as u32 - 1) {
                    4 => {
                        match checksum(card_number, card_number_len) {
                            true => stat = Card::VISA,
                            false => ()
                        }
                    },
                    5 => match card_number / 10_u64.pow(card_number_len as u32 - 2) {
                        51 | 52 | 53 | 54 | 55 => {
                            match checksum(card_number, card_number_len) {
                                true => stat = Card::MasterCard,
                                false => ()
                            }
                        },
                        _ => ()
                    }
                    _ => ()
                }
            },
            _ => ()
        }

        match stat {
            Card::VISA => println!("It's a valid VISA credit card."),
            Card::MasterCard => println!("It's a valid MasterCard credit card."),
            Card::AMEX => println!("It's a valid American Express credit card."),
            Card::INVALID => println!("It's not a valid card number!")
        }
    }
}

fn checksum(number: u64, len: usize) -> bool {
    // Put the card number alternately into two sums digit by digit
    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;
    let mut card_number = number;
    let mut counter = 0;
    let mut extractor = 10;

    while counter < len {
        match counter % 2 {
            1 => {
                sum_1 += card_number % extractor;
                card_number -= card_number % extractor;
            },
            _ => {
                sum_2 += card_number % extractor;
                card_number -= card_number % extractor;
            }
        }
        extractor *= 10;
        counter += 1
    }

    // Calculate the checksum
    let mut checksum = 0;
    
    sum_1 *= 2;
    while sum_1 >= 1 {
        checksum += sum_1 % 10;
        sum_1 /= 10;
    }

    while sum_2 >= 1 {
        checksum += sum_2 % 10;
        sum_2 /= 10;
    }

    // Return true if the checksum suggests the card is valid
    match checksum % 10 {
        0 => return true,
        _ => return false
    }
}