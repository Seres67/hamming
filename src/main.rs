use std::env;

/**
 * height and width = sqrt(package size)
 * number of parity bits = log2(package size)
 * parity bits are: 1, 2, 4, 8, 16, 32, 64, 128, 256
 * number of data bits = package size - number of parity bits
 * minimum package length = 16
 **/

fn encode_message_length(size: usize) -> [bool; 16] {
    let mut binary_size = [false; 11];
    for (i, bit) in format!("{size:011b}").chars().enumerate() {
        binary_size[i] = bit == '1';
    }
    let mut length_pack = [
        false,
        false,
        false,
        binary_size[0],
        false,
        binary_size[1],
        binary_size[2],
        binary_size[3],
        false,
        binary_size[4],
        binary_size[5],
        binary_size[6],
        binary_size[7],
        binary_size[8],
        binary_size[9],
        binary_size[10],
    ];

    length_pack[1] = length_pack
        .iter()
        .skip(3)
        .step_by(2)
        .filter(|&&bit| bit)
        .count()
        % 2
        == 1;
    let mut count = 0;
    for i in [3, 6, 7, 10, 11, 14, 15] {
        count += if length_pack[i] { 1 } else { 0 };
    }
    length_pack[2] = count % 2 == 1;
    count = 0;
    for i in [5, 6, 7, 12, 13, 14, 15] {
        count += if length_pack[i] { 1 } else { 0 };
    }
    length_pack[4] = count % 2 == 1;
    length_pack[8] = length_pack
        .iter()
        .skip(9)
        .step_by(1)
        .filter(|&&bit| bit)
        .count()
        % 2
        == 1;
    length_pack[0] = length_pack.iter().fold(false, |acc, &bit| acc ^ bit);
    length_pack
}

fn encode_message(message: &String) -> Vec<bool> {
    let length = (message.len() * 8).next_power_of_two();
    let mut binary_message = String::new();
    for character in message.clone().into_bytes() {
        binary_message += &format!("{character:08b}");
    }
    let mut message_pack: Vec<bool> = Vec::with_capacity(length);
    let mut binary_chars = binary_message.chars();
    for i in 0..length {
        if i == 0 || i.is_power_of_two() {
            message_pack.push(false);
            continue;
        }
        let char = binary_chars.next();
        if let Some(..) = char {
            let c = char.unwrap();
            message_pack.push(c == '1');
        } else {
            message_pack.push(false);
        }
    }
    message_pack[1] = message_pack.iter().skip(3).step_by(2).count() % 2 == 1;
    let mut count = 0;
    let mut i = 3;
    while i < length {
        count += if message_pack[i] { 1 } else { 0 };
        if i % 2 == 1 {
            i += 2;
        }
        i += 1;
    }
    message_pack[2] = count % 2 == 1;
    count = 0;
    i = 5;
    while i < length {
        count += if message_pack[i] { 1 } else { 0 };
        if (i + 1) % 8 == 0 {
            i += 4;
        }
        i += 1;
    }
    message_pack[4] = count % 2 == 1;
    count = 0;
    i = 9;
    while i < length {
        count += if message_pack[i] { 1 } else { 0 };
        if (i + 1) % 8 == 0 {
            i += 8;
        }
        i += 1;
    }
    message_pack[8] = count % 2 == 1;
    count = 0;
    i = 17;
    while i < length {
        count += if message_pack[i] { 1 } else { 0 };
        if (i + 1) % 32 == 0 {
            i += 16;
        }
        i += 1;
    }
    message_pack[16] = count % 2 == 1;
    count = 0;
    i = 33;
    while i < length {
        count += if message_pack[i] { 1 } else { 0 };
        i += 1;
    }
    message_pack[32] = count % 2 == 1;
    count = 0;
    i = 0;
    while i < length {
        count += if message_pack[i] { 1 } else { 0 };
        i += 1;
    }
    message_pack[0] = count % 2 == 1;
    message_pack
}

fn main() {
    let message = match env::args().nth(1) {
        Some(msg) => msg,
        None => {
            eprintln!("Usage: hamming <message>");
            std::process::exit(1);
        }
    };
    let length_pack = encode_message_length(message.len());
    for (i, bit) in length_pack.iter().enumerate() {
        if i % 4 == 0 {
            println!();
        }
        print!("{}", if *bit { "1" } else { "0" });
    }
    let message_pack = encode_message(&message);
    for (i, bit) in message_pack.iter().enumerate() {
        if i % 8 == 0 {
            println!();
        }
        print!("{}", if *bit { "1" } else { "0" });
    }
    print!("\n===========");
    let mut binary_message = String::new();
    for character in message.into_bytes() {
        binary_message += &format!("{character:08b}");
    }
}
