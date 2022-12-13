use std::io;

fn main() {
    let line = 40;
    let mut pos = 0;

    let mut reg_x = 1;
    let mut str_x = reg_x;
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
                &_ => todo!(),
            }
        }

        if pos >= reg_x - 1 && pos <= reg_x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        pos += 1;
        if pos >= line {
            println!("");
            pos = 0;
        }

        busy_cycles -= 1;
    }
}
