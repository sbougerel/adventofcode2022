use std::collections::VecDeque;

fn default_worry_manager(worry: i64) -> i64 {
    worry / 3
}

pub struct Monkey {
    operation: Box<dyn Fn(i64) -> i64 + 'static>,
    test_value: i64,
    dest_true: usize,
    dest_false: usize,
    worry_manager: Box<dyn Fn(i64) -> i64 + 'static>,
    inspected: i64,
}

impl Monkey {
    pub fn new(
        operation: Box<dyn Fn(i64) -> i64 + 'static>,
        test_value: i64,
        dest_true: usize,
        dest_false: usize,
    ) -> Monkey {
        Monkey {
            operation: operation,
            test_value: test_value,
            dest_true: dest_true,
            dest_false: dest_false,
            worry_manager: Box::new(default_worry_manager),
            inspected: 0,
        }
    }

    pub fn new_worry(
        operation: Box<dyn Fn(i64) -> i64 + 'static>,
        test_value: i64,
        dest_true: usize,
        dest_false: usize,
        worry_manager: Box<dyn Fn(i64) -> i64 + 'static>,
    ) -> Monkey {
        Monkey {
            operation: operation,
            test_value: test_value,
            dest_true: dest_true,
            dest_false: dest_false,
            worry_manager: worry_manager,
            inspected: 0,
        }
    }

    pub fn play(&mut self, id: usize, queues: &mut Vec<VecDeque<i64>>) {
        while let Some(mut worry) = queues[id].pop_front() {
            worry = (self.operation)(worry);
            worry = (self.worry_manager)(worry);
            if worry % self.test_value == 0 {
                queues[self.dest_true].push_front(worry);
            } else {
                queues[self.dest_false].push_front(worry);
            }
            self.inspected += 1;
        }
    }
}

pub struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    queues: Vec<VecDeque<i64>>,
}

impl MonkeyBusiness {
    pub fn new() -> MonkeyBusiness {
        MonkeyBusiness {
            monkeys: Vec::new(),
            queues: Vec::new(),
        }
    }

    pub fn add(&mut self, items: VecDeque<i64>, monkey: Monkey) {
        self.monkeys.push(monkey);
        self.queues.push(items);
    }

    pub fn play(&mut self, rounds: usize) {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() {
                self.monkeys[i].play(i, &mut self.queues);
            }
        }
    }

    pub fn most_active(self) -> (i64, i64) {
        let mut activity = self
            .monkeys
            .iter()
            .map(|x| x.inspected)
            .collect::<Vec<i64>>();
        let len = activity.len();
        activity.select_nth_unstable(len - 2);
        (activity[len - 1], activity[len - 2])
    }
}
