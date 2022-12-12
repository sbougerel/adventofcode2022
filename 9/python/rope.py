#!/usr/bin/env python3

import sys
from numpy import array

rope_len = int(sys.argv[1])
rope = [array([0, 0]) for i in range(rope_len)]
visited = {tuple(rope[-1])}
move_map = {
    "D": array([0, -1]),
    "U": array([0, 1]),
    "L": array([-1, 0]),
    "R": array([1, 0]),
}

for line in sys.stdin:
    orient, steps = line.split(" ")
    for i in range(int(steps)):
        rope[0] += move_map[orient]
        for knot in range(1, len(rope)):
            dt = rope[knot - 1] - rope[knot]
            if 2 < sum(dt ** 2):
                rope[knot] += (dt > 0).astype(int) - (dt < 0)
            else:
                break
        visited.add(tuple(rope[-1]))
print(len(visited))
