#!/usr/bin/env python3


def default_worry_manager(worry):
    return int(worry / 3)


class Monkey:
    def __init__(
        self,
        queue,
        operation,
        test_value,
        dest_true,
        dest_false,
        worry_manager=default_worry_manager,
    ) -> None:
        self.queue = queue
        self.operation = operation
        self.test_value = test_value
        self.dest_true = dest_true
        self.dest_false = dest_false
        self.worry_manager = worry_manager
        self.inspected = 0

    def do(self, monkeys):
        while len(self.queue) > 0:
            worry = self.queue[0]
            del self.queue[0]
            worry = self.operation(worry)
            worry = self.worry_manager(worry)
            if worry % self.test_value == 0:
                monkeys[self.dest_true].queue.append(worry)
            else:
                monkeys[self.dest_false].queue.append(worry)
            self.inspected += 1


def do_monkey_business(rounds, monkeys):
    for _ in range(rounds):
        for monkey in monkeys:
            monkey.do(monkeys)


def most_active_monkeys(monkeys):
    monkey_activity = [monkey.inspected for monkey in monkeys]
    return tuple(sorted(monkey_activity, reverse=True)[:2])
