#!/usr/bin/env python3
from monkey import Monkey, MonkeyBusiness


def worry_manager(worry):
    return worry % (23 * 19 * 13 * 17)


monkey_business = MonkeyBusiness(
    Monkey(
        queue=[79, 98],
        operation=lambda worry: worry * 19,
        test_value=23,
        dest_true=2,
        dest_false=3,
        worry_manager=worry_manager,
    ),
    Monkey(
        queue=[54, 65, 75, 74],
        operation=lambda worry: worry + 6,
        test_value=19,
        dest_true=2,
        dest_false=0,
        worry_manager=worry_manager,
    ),
    Monkey(
        queue=[79, 60, 97],
        operation=lambda worry: worry**2,
        test_value=13,
        dest_true=1,
        dest_false=3,
        worry_manager=worry_manager,
    ),
    Monkey(
        queue=[74],
        operation=lambda worry: worry + 3,
        test_value=17,
        dest_true=0,
        dest_false=1,
        worry_manager=worry_manager,
    ),
)

monkey_business.do(10000)
x, y = monkey_business.most_active()
print(f"{x} * {y} = {x * y}")
