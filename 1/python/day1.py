#!/usr/bin/env python3
import sys

# Solved for speed, so ugly code with copy/pasted portion.

sums = 0
max_sums = [0, 0, 0]


def store():
    index = len(max_sums)
    for i, v in enumerate(max_sums):
        if sums >= v:
            index = i
            break
    max_sums.insert(index, sums)
    max_sums.pop()


for line in sys.stdin:
    line = line.strip()
    if line == "":
        store()
        sums = 0
    else:
        sums += int(line)
# One last time, after EOF, for good measure
store()

print(f"Part 1: {max_sums[0]}")
print("Part 2: {} = {}".format(max_sums, sum(max_sums)))
