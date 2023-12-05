use std::str::FromStr;

use strum::{EnumString, EnumVariantNames};

#[derive(Debug, EnumString, EnumVariantNames, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum Color {
    Blue,
    Red,
    Green,
}

fn parse_color_field(input: &str) -> Option<(usize, Color)> {
    let input = input.trim();
    input.find(' ').and_then(|i| {
        match (
            (&input[0..i]).parse::<usize>(),
            Color::from_str(&input[(i + 1)..]),
        ) {
            (Ok(a), Ok(b)) => Some((a, b)),
            _ => {
                dbg!("Error Case");
                None
            }
        }
    })
}

fn main() -> std::io::Result<()> {
    let mut game_power_sum = 0;

    let mut buf = String::new();

    while std::io::stdin().read_line(&mut buf)? != 0 {
        let index_of_colon = match buf.find(':') {
            Some(index) => index,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unable to find ':'",
                ))
            }
        };

        let mut min_blue = 0;
        let mut min_red = 0;
        let mut min_green = 0;

        //let game_number = match (&buf[5..index_of_colon]).parse::<usize>() {
        //    Ok(val) => val,
        //    Err(err) => {
        //        return Err(std::io::Error::new(
        //            std::io::ErrorKind::InvalidData,
        //            format!("Unable to find game number. {}", err),
        //        ))
        //    }
        //};

        let colors_and_quantity: Vec<Vec<_>> = match (&buf[(index_of_colon + 1)..])
            .split(';')
            .map(|s| s.split(',').map(|ss| parse_color_field(ss)).collect())
            .collect()
        {
            Some(x) => x,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Error parsing color fields",
                ))
            }
        };

        for draw in colors_and_quantity {
            for (num, color) in draw {
                if color == Color::Blue {
                    min_blue = usize::max(min_blue, num);
                } else if color == Color::Red {
                    min_red = usize::max(min_red, num);
                } else if color == Color::Green {
                    min_green = usize::max(min_green, num);
                }
            }
        }

        let power = min_green * min_red * min_blue;

        game_power_sum += power;

        buf.clear();
    }

    println!("{}", game_power_sum);

    Ok(())
}
