use std::collections::VecDeque;

use monkey::common::{Monkey, MonkeyBusiness};

fn main() {
    let mut monkey_business = MonkeyBusiness::new();
    monkey_business.add(
        VecDeque::from([52, 78, 79, 63, 51, 94]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry * 13 }), 5, 1, 6),
    );
    monkey_business.add(
        VecDeque::from([77, 94, 70, 83, 53]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry + 3 }), 7, 5, 3),
    );
    monkey_business.add(
        VecDeque::from([98, 50, 76]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry * worry }), 13, 0, 6),
    );
    monkey_business.add(
        VecDeque::from([92, 91, 61, 75, 99, 63, 84, 69]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry + 5 }), 11, 5, 7),
    );
    monkey_business.add(
        VecDeque::from([51, 53, 83, 52]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry + 7 }), 3, 2, 0),
    );
    monkey_business.add(
        VecDeque::from([76, 76]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry + 4 }), 2, 4, 7),
    );
    monkey_business.add(
        VecDeque::from([75, 59, 93, 69, 76, 96, 65]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry * 19 }), 17, 1, 3),
    );
    monkey_business.add(
        VecDeque::from([89]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry + 2 }), 19, 2, 4),
    );

    monkey_business.play(20);
    let (x, y) = monkey_business.most_active();
    let xy = x * y;
    println!("{x} * {y} = {xy}");
}
