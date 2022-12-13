#!/usr/bin/env python3
import sys

line = 40
pos = 0

reg_x = 1
str_x = reg_x
cycle = 1
busy_cycles = 0

while True:
    if busy_cycles == 0:
        reg_x = str_x
        tokens = sys.stdin.readline().split()
        if len(tokens) == 0:
            break  # done
        if tokens[0] == "noop":
            busy_cycles = 1
            str_x = reg_x
        elif tokens[0] == "addx":
            busy_cycles = 2
            str_x = reg_x + int(tokens[1])

    if pos >= reg_x - 1 and pos <= reg_x + 1:
        print("#", end="")
    else:
        print(".", end="")
    pos += 1
    if pos >= line:
        print()
        pos = 0

    cycle += 1
    busy_cycles -= 1

print()
