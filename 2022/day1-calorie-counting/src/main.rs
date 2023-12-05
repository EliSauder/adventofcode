fn main() {
    let mut input_buf = String::new();

    let mut current_total = 0;
    let mut current_max = 0;

    let mut max_stack = Vec::new();

    while match std::io::stdin().read_line(&mut input_buf) {
        Ok(bytes_read) => bytes_read,
        Err(error) => panic!("Error {:?}", error),
    } != 0
    {
        if input_buf.trim().len() == 0 {
            if current_total > current_max {
                max_stack.push(current_total);
                current_max = current_total;
            }
            current_total = 0;
            continue;
        }

        let input = match input_buf.trim().parse::<i32>() {
            Ok(val) => val,
            Err(error) => panic!("Error {:?}", error),
        };

        current_total += input;

        input_buf.clear();
    }

    let mut count = 3;
    let mut sum = 0;
    print!("Top Maxes:\n\t");
    while let Some(i) = max_stack.pop() {
        if count <= 0 {
            break;
        }

        sum += i;
        print!("{},", i);
        count -= 1;
    }
    println!("\nTotal: {}", sum);
}
