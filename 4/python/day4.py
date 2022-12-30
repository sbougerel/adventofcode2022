#!/usr/bin/env python3
import sys

I = []
for l in sys.stdin:
    l.strip()
    p1, p2 = l.split(",")
    lm, lM = p1.split("-")
    rm, rM = p2.split("-")
    lm, lM, rm, rM = int(lm), int(lM), int(rm), int(rM)
    I.append((lm, lM, rm, rM))

sP = 0
for lm, lM, rm, rM in I:
    if lm <= rm and rM <= lM or rm <= lm and lM <= rM:
        sP += 1

print(f"Part1: {sP}")


sP = 0
for lm, lM, rm, rM in I:
    if (
        lm <= rm
        and rm <= lM
        or lm <= rM
        and rM <= lM
        or rm <= lm
        and lm <= rM
        or rm <= lM
        and lM <= rM
    ):
        sP += 1

print(f"Part2: {sP}")
