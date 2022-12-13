use std::collections::VecDeque;

use monkey::common::{Monkey, MonkeyBusiness};

fn main() {
    let mut monkey_business = MonkeyBusiness::new();
    monkey_business.add(
        VecDeque::from([79, 98]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry * 19 }), 23, 2, 3),
    );
    monkey_business.add(
        VecDeque::from([54, 65, 75, 74]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry + 6 }), 19, 2, 0),
    );
    monkey_business.add(
        VecDeque::from([79, 60, 97]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry * worry }), 13, 1, 3),
    );
    monkey_business.add(
        VecDeque::from([74]),
        Monkey::new(Box::new(|worry: i64| -> i64 { worry + 3 }), 17, 0, 1),
    );

    monkey_business.play(20);
    let (x, y) = monkey_business.most_active();
    let xy = x * y;
    println!("{x} * {y} = {xy}");
}
