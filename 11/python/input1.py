#!/usr/bin/env python3
from monkey import Monkey, MonkeyBusiness

monkey_business = MonkeyBusiness(
    Monkey(
        queue=[52, 78, 79, 63, 51, 94],
        operation=lambda worry: worry * 13,
        test_value=5,
        dest_true=1,
        dest_false=6,
    ),
    Monkey(
        queue=[77, 94, 70, 83, 53],
        operation=lambda worry: worry + 3,
        test_value=7,
        dest_true=5,
        dest_false=3,
    ),
    Monkey(
        queue=[98, 50, 76],
        operation=lambda worry: worry**2,
        test_value=13,
        dest_true=0,
        dest_false=6,
    ),
    Monkey(
        queue=[92, 91, 61, 75, 99, 63, 84, 69],
        operation=lambda worry: worry + 5,
        test_value=11,
        dest_true=5,
        dest_false=7,
    ),
    Monkey(
        queue=[51, 53, 83, 52],
        operation=lambda worry: worry + 7,
        test_value=3,
        dest_true=2,
        dest_false=0,
    ),
    Monkey(
        queue=[76, 76],
        operation=lambda worry: worry + 4,
        test_value=2,
        dest_true=4,
        dest_false=7,
    ),
    Monkey(
        queue=[75, 59, 93, 69, 76, 96, 65],
        operation=lambda worry: worry * 19,
        test_value=17,
        dest_true=1,
        dest_false=3,
    ),
    Monkey(
        queue=[89],
        operation=lambda worry: worry + 2,
        test_value=19,
        dest_true=2,
        dest_false=4,
    ),
)

monkey_business.do(20)
x, y = monkey_business.most_active()
print(f"{x} * {y} = {x * y}")
