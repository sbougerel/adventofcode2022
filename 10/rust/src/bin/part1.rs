use std::io;

fn main() {
    let sig_inter = 40;
    let mut sig_peek = 20;
    let mut sig_sum = 0;

    let mut reg_x = 1;
    let mut str_x = reg_x;
    let mut cycle = 1;
    let mut busy_cycles = 0;

    loop {
        if busy_cycles == 0 {
            reg_x = str_x;
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.is_empty() {
                break;
            }
            let tokens = input.split(" ").collect::<Vec<&str>>();
            match tokens[0].trim() {
                "noop" => {
                    busy_cycles = 1;
                    str_x = reg_x;
                }
                "addx" => {
                    busy_cycles = 2;
                    str_x = reg_x + tokens[1].trim().parse::<i32>().unwrap();
                }
                &_ => {
                    todo!();
                }
            }
        }

        if cycle == sig_peek {
            sig_peek += sig_inter;
            sig_sum += reg_x * cycle;
        }

        cycle += 1;
        busy_cycles -= 1;
    }

    println!("{sig_sum}");
}
