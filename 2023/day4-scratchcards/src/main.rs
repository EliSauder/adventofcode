use std::collections::HashSet;

fn main() -> std::result::Result<(), std::io::Error> {
    let mut input_buf = String::new();

    let mut counts = Vec::new();

    while match std::io::stdin().read_line(&mut input_buf) {
        Ok(bytes_read) => bytes_read,
        Err(err) => return Err(err),
    } != 0
    {
        let input: Vec<HashSet<i32>> = match (&input_buf[9..])
            .split('|')
            .map(|s| s.split_whitespace().map(|ss| ss.parse::<i32>()).collect())
            .collect()
        {
            Ok(vals) => vals,
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{}", err),
                ))
            }
        };

        let intersection = input[0].intersection(&input[1]);
        let intersection_count = intersection.count();
        counts.push(intersection_count);

        input_buf.clear();
    }

    let mut cards = Vec::new();
    cards.resize(counts.len(), 0);

    for i in 0..cards.len() {
        cards[i] += 1;
        for j in (i + 1)..=(i + counts[i]) {
            if j >= cards.len() {
                break;
            }
            cards[j] += cards[i];
        }
    }

    let total_num_cards: usize = cards.iter().sum();

    println!("Num Cards: {}", total_num_cards);

    Ok(())
}
