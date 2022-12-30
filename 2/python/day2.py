#!/usr/bin/env python3
import sys

# Solved for speed, so ugly code with copy/pasted portion.

sums = 0
max_sums = [0, 0, 0]

# First: A for Rock, B for Paper, and C for Scissors.
# Response: X for Rock, Y for Paper, and Z for Scissors.
# Bonus: 1 for Rock, 2 for Paper, and 3 for Scissors.

# (0 if you lost, 3 if the round was a draw, and 6 if you won).
part1_points = {
    "A X": 3 + 1,
    "A Y": 6 + 2,
    "A Z": 0 + 3,
    "B X": 0 + 1,
    "B Y": 3 + 2,
    "B Z": 6 + 3,
    "C X": 6 + 1,
    "C Y": 0 + 2,
    "C Z": 3 + 3,
}

#  X means you need to lose, Y means you need to end the round in a draw, and Z
#  means you need to win.
part2_points = {
    "A X": 0 + 3,
    "A Y": 3 + 1,
    "A Z": 6 + 2,
    "B X": 0 + 1,
    "B Y": 3 + 2,
    "B Z": 6 + 3,
    "C X": 0 + 2,
    "C Y": 3 + 3,
    "C Z": 6 + 1,
}

S1 = 0
S2 = 0
for line in sys.stdin:
    line = line.strip()
    S1 += part1_points[line]
    S2 += part2_points[line]

print(f"Part 1: {S1}")
print(f"Part 2: {S2}")
