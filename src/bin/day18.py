from collections import Counter
from dataclasses import dataclass
from typing import List, Set


@dataclass(frozen=True)
class Point:
    x: int
    y: int
    z: int


with open("../../data/day18.txt", "r") as f:
    lines = f.readlines()
    lines = [i.strip("\n") for i in lines]
    points: Set[Point] = {Point(*[int(j) for j in i.split(",")]) for i in lines}
sides_x = Counter([i.x for i in points])
sides_y = Counter([i.y for i in points])
sides_z = Counter([i.z for i in points])


def manhattan_distance(p1, p2) -> int:
    return abs(p1.x - p2.x) + abs(p1.y - p2.y) + abs(p1.z - p2.z)


nsides = len(points) * 6
overlaps = 0
for p in points:
    for q in points:
        if manhattan_distance(p, q) == 1:
            overlaps += 1
part1 = nsides - overlaps
print(part1)
# Part 2
# Need to find holes in the grid
min_x = min([i.x for i in points])
max_x = max([i.x for i in points])
min_y = min([i.y for i in points])
max_y = max([i.y for i in points])
min_z = min([i.z for i in points])
max_z = max([i.z for i in points])
print(min_x, max_x, min_y, max_y, min_z, max_z)
empty_points = set()
for x in range(min_x - 1, max_x + 2):
    for y in range(min_y - 1, max_y + 2):
        for z in range(min_z - 1, max_z + 2):
            if Point(x, y, z) not in points:
                print(x, y, z)
                empty_points.add(Point(x, y, z))
visited = set()
accessible_points = [Point(4, 4, 4)]
while len(accessible_points) > 0:
    point = accessible_points.pop()
    # Explore the neighbors of the point, marking points as visited
    if (
        point.x + 1 <= max_x + 1
        and Point(point.x + 1, point.y, point.z) not in points
        and Point(point.x + 1, point.y, point.z) not in visited
    ):
        accessible_points.append(Point(point.x + 1, point.y, point.z))
        visited.add(Point(point.x + 1, point.y, point.z))
    if (
        point.x - 1 >= min_x - 1
        and Point(point.x - 1, point.y, point.z) not in points
        and Point(point.x - 1, point.y, point.z) not in visited
    ):
        accessible_points.append(Point(point.x - 1, point.y, point.z))
        visited.add(Point(point.x - 1, point.y, point.z))
    if (
        point.y + 1 <= max_y + 1
        and Point(point.x, point.y + 1, point.z) not in points
        and Point(point.x, point.y + 1, point.z) not in visited
    ):
        accessible_points.append(Point(point.x, point.y + 1, point.z))
        visited.add(Point(point.x, point.y + 1, point.z))
    if (
        point.y - 1 >= min_y - 1
        and Point(point.x, point.y - 1, point.z) not in points
        and Point(point.x, point.y - 1, point.z) not in visited
    ):
        accessible_points.append(Point(point.x, point.y - 1, point.z))
        visited.add(Point(point.x, point.y - 1, point.z))
    if (
        point.z + 1 <= max_z + 1
        and Point(point.x, point.y, point.z + 1) not in points
        and Point(point.x, point.y, point.z + 1) not in visited
    ):
        accessible_points.append(Point(point.x, point.y, point.z + 1))
        visited.add(Point(point.x, point.y, point.z + 1))
    if (
        point.z - 1 >= min_z - 1
        and Point(point.x, point.y, point.z - 1) not in points
        and Point(point.x, point.y, point.z - 1) not in visited
    ):
        accessible_points.append(Point(point.x, point.y, point.z - 1))
        visited.add(Point(point.x, point.y, point.z - 1))
print("POCKETS")
print(empty_points - visited)
original_pockets = list(empty_points - visited)
pockets = list(empty_points - visited)
total_overlaps = 0
while len(pockets) > 0:
    point = pockets.pop()
    candidate_unified_pocket = set([point])
    accessible_from_this_pocket = [point]
    while len(accessible_from_this_pocket) > 0:
        point = accessible_from_this_pocket.pop()
        # Explore the neighbors of the point, marking points as candidate_unified_pocket
        # All the points we explore are in original_pockets
        if (
            Point(point.x + 1, point.y, point.z) in original_pockets
            and Point(point.x + 1, point.y, point.z) not in candidate_unified_pocket
        ):
            candidate_unified_pocket.add(Point(point.x + 1, point.y, point.z))
            accessible_from_this_pocket.append(Point(point.x + 1, point.y, point.z))
            pockets.remove(Point(point.x + 1, point.y, point.z))
        if (
            Point(point.x - 1, point.y, point.z) not in points
            and Point(point.x - 1, point.y, point.z) not in candidate_unified_pocket
        ):
            candidate_unified_pocket.add(Point(point.x - 1, point.y, point.z))
            accessible_from_this_pocket.append(Point(point.x - 1, point.y, point.z))
            pockets.remove(Point(point.x - 1, point.y, point.z))
        if (
            Point(point.x, point.y + 1, point.z) not in points
            and Point(point.x, point.y + 1, point.z) not in candidate_unified_pocket
        ):
            candidate_unified_pocket.add(Point(point.x, point.y + 1, point.z))
            accessible_from_this_pocket.append(Point(point.x, point.y + 1, point.z))
            pockets.remove(Point(point.x, point.y + 1, point.z))
        if (
            Point(point.x, point.y - 1, point.z) not in points
            and Point(point.x, point.y - 1, point.z) not in candidate_unified_pocket
        ):
            candidate_unified_pocket.add(Point(point.x, point.y - 1, point.z))
            accessible_from_this_pocket.append(Point(point.x, point.y - 1, point.z))
            pockets.remove(Point(point.x, point.y - 1, point.z))
        if (
            Point(point.x, point.y, point.z + 1) not in points
            and Point(point.x, point.y, point.z + 1) not in candidate_unified_pocket
        ):
            candidate_unified_pocket.add(Point(point.x, point.y, point.z + 1))
            accessible_from_this_pocket.append(Point(point.x, point.y, point.z + 1))
            pockets.remove(Point(point.x, point.y, point.z + 1))
        if (
            Point(point.x, point.y, point.z - 1) not in points
            and Point(point.x, point.y, point.z - 1) not in candidate_unified_pocket
        ):
            candidate_unified_pocket.add(Point(point.x, point.y, point.z - 1))
            accessible_from_this_pocket.append(Point(point.x, point.y, point.z - 1))
            pockets.remove(Point(point.x, point.y, point.z - 1))
    nsides = len(candidate_unified_pocket) * 6
    overlaps = 0
    for p in candidate_unified_pocket:
        for q in candidate_unified_pocket:
            if manhattan_distance(p, q) == 1:
                overlaps += 1
    total_overlaps += nsides - overlaps
print(part1 - total_overlaps)
