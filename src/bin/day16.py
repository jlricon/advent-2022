# https://github.com/juanplopes/advent-of-code-2022/blob/main/day16.py
import sys, re
from typing import Dict

with open("/Users/jlricon/gits/advent-2022/data/day16.txt", "r") as f:
    lines = [re.split("[\\s=;,]+", x) for x in f.readlines()]
G = {x[1]: set(x[10:]) for x in lines}
F = {x[1]: int(x[5]) for x in lines if int(x[5]) != 0}
I = {x: 1 << i for i, x in enumerate(F)}
T = {x: {y: 1 if y in G[x] else float("+inf") for y in G} for x in G}

for k in T:
    for i in T:
        for j in T:
            T[i][j] = min(T[i][j], T[i][k] + T[k][j])
print(I)


def visit(current_valve: str, time_remaining: int, state: int, flow: int, answer: Dict[int, int]):
    # The answer for a given state (set of valve opens)
    answer[state] = max(answer.get(state, 0), flow)
    for valve in F:
        minutes = time_remaining - T[current_valve][valve] - 1
        if I[valve] & state or minutes <= 0:
            continue
        visit(valve, int(minutes), state | I[valve], flow + int(minutes) * F[valve], answer)
    return answer


print(visit("AA", 30, 0, 0, {}))
total1 = max(visit("AA", 30, 0, 0, {}).values())
visited2 = visit("AA", 26, 0, 0, {})
print(visited2)
total2 = max(v1 + v2 for k1, v1 in visited2.items() for k2, v2 in visited2.items() if not k1 & k2)
print(total1, total2)
