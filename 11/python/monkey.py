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

    def do(self, queues):
        while len(self.queue) > 0:
            worry = self.queue[0]
            del self.queue[0]
            worry = self.operation(worry)
            worry = self.worry_manager(worry)
            if worry % self.test_value == 0:
                queues[self.dest_true].append(worry)
            else:
                queues[self.dest_false].append(worry)
            self.inspected += 1


class MonkeyBusiness:
    def __init__(self, *monkeys) -> None:
        self.monkeys = []
        self.destinations = []
        self.add(*monkeys)

    def add(self, *monkeys):
        for monkey in monkeys:
            self.monkeys.append(monkey)
            self.destinations.append(monkey.queue)

    def do(self, rounds):
        for _ in range(rounds):
            for monkey in self.monkeys:
                monkey.do(self.destinations)

    def most_active(self):
        monkey_activity = [monkey.inspected for monkey in self.monkeys]
        return tuple(sorted(monkey_activity, reverse=True)[:2])
