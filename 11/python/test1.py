#!/usr/bin/env python3
from monkey import Monkey, do_monkey_business, most_active_monkeys

monkeys = [
    Monkey(
        queue=[79, 98],
        operation=lambda worry: worry * 19,
        test_value=23,
        dest_true=2,
        dest_false=3,
    ),
    Monkey(
        queue=[54, 65, 75, 74],
        operation=lambda worry: worry + 6,
        test_value=19,
        dest_true=2,
        dest_false=0,
    ),
    Monkey(
        queue=[79, 60, 97],
        operation=lambda worry: worry**2,
        test_value=13,
        dest_true=1,
        dest_false=3,
    ),
    Monkey(
        queue=[74],
        operation=lambda worry: worry + 3,
        test_value=17,
        dest_true=0,
        dest_false=1,
    ),
]

do_monkey_business(20, monkeys)
x, y = most_active_monkeys(monkeys)
print(f"{x} * {y} = {x * y}")
