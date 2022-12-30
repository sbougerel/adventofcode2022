#!/usr/bin/env python3

import sys

# Lowercase item types a through z have priorities 1 through 26.
# Uppercase item types A through Z have priorities 27 through 52.

I = []
for l in sys.stdin:
    I.append(l.strip())


S = 0
C: set[int]
for s in I:
    C: set[int] = set([c for c in s[: len(s) >> 1]])
    for c in s[len(s) >> 1 :]:
        if c in C:
            if ord(c) >= ord("A") and ord("Z") >= ord(c):
                S += ord(c) - ord("A") + 27
            else:
                S += ord(c) - ord("a") + 1
            break

print(f"Part1: {S}")

S = 0
D: dict[str, int] = {}
for i, s in enumerate(I):
    if i % 3 == 0:
        D = dict([(c, 1) for c in s])
    elif i % 3 == 1:
        for c in s:
            if c in D:
                D[c] = 2
    elif i % 3 == 2:
        for c in s:
            if c in D and D[c] == 2:
                if ord(c) >= ord("A") and ord("Z") >= ord(c):
                    S += ord(c) - ord("A") + 27
                else:
                    S += ord(c) - ord("a") + 1
                break

print(f"Part2: {S}")
