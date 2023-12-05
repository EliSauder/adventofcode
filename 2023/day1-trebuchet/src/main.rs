fn get_value_digit(val: &str) -> Option<i32> {
    match val {
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        s if s.starts_with("zero") => Some(0),
        _ => None,
    }
}

fn get_digit(val: &str) -> Option<i32> {
    if let Some(c) = val.chars().nth(0) {
        if char::is_numeric(c) {
            dbg!("Found digit {}", c as i32 - '0' as i32);
            return Some(c as i32 - '0' as i32);
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let mut sum = 0;

    while match std::io::stdin().read_line(&mut buf) {
        Ok(bytes) => bytes,
        Err(err) => return Err(err),
    } != 0
    {
        let mut first_val = None;
        let mut first_dig = None;

        let mut last_val = None;
        let mut last_dig = None;

        dbg!("Processing: {}", &buf);

        for i in 0..buf.len() + 1 {
            let first_view = &buf[i..usize::min(i + 5, buf.len())];
            let last_view = &buf[(buf.len() - i)..usize::min(buf.len() + 5 - i, buf.len())];

            dbg!("First View: {}, Last View: {}", first_view, last_view);

            if first_val.is_none() && first_dig.is_none() {
                first_val = get_value_digit(first_view);
                first_dig = get_digit(first_view);
            }

            if last_val.is_none() && last_dig.is_none() {
                last_val = get_value_digit(last_view);
                last_dig = get_digit(last_view);
            }

            if (first_val.is_some() || first_dig.is_some())
                && (last_val.is_some() || last_dig.is_some())
            {
                break;
            }
        }

        let first_digit = if let Some(first) = first_val {
            Some(first)
        } else if let Some(first) = first_dig {
            Some(first)
        } else {
            None
        };

        let last_digit = if let Some(last) = last_val {
            Some(last)
        } else if let Some(last) = last_dig {
            Some(last)
        } else {
            None
        };

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            dbg!("Found Vals: {} and {}", first, last);
            sum += first * 10 + last;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unable to find both first and last digit",
            ));
        }

        buf.clear();
    }

    println!("{}", sum);

    Ok(())
}
