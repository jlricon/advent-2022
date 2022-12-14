from dataclasses import dataclass
from itertools import chain
import math
from typing import Dict, Tuple


@dataclass
class Point:
    x: int
    y: int


@dataclass
class Segment:
    start: Point
    end: Point

    def points_in_between(self) -> list[Point]:
        """
        Returns all points in between start and end
        """
        points = []

        if self.start.x == self.end.x:
            increment = int(math.copysign(1, self.end.y - self.start.y))
            for y in range(self.start.y, self.end.y, increment):
                points.append(Point(self.start.x, y))
        elif self.start.y == self.end.y:
            increment = int(math.copysign(1, self.end.x - self.start.x))
            for x in range(self.start.x, self.end.x, increment):
                points.append(Point(x, self.start.y))
        else:
            raise ValueError("Segment is not horizontal or vertical")
        return points


assert Segment(Point(498, 4), Point(498, 6)).points_in_between() == [
    Point(498, 4),
    Point(498, 5),
]
assert Segment(Point(498, 6), Point(496, 6)).points_in_between() == [
    Point(498, 6),
    Point(497, 6),
]


@dataclass
class Line:
    segments: list[Segment]

    def points_in_between(self) -> list[Point]:
        points = []
        for segment in self.segments:
            points.extend(segment.points_in_between())
        points.append(self.segments[-1].end)
        return points

    @staticmethod
    def new(line: list[str]) -> "Line":
        segment_list = [i.split(",") for i in line]
        segments = []
        for i in range(len(segment_list) - 1):
            first_point = segment_list[i]
            second_point = segment_list[i + 1]
            segments.append(
                Segment(
                    Point(int(first_point[0]), int(first_point[1])),
                    Point(int(second_point[0]), int(second_point[1])),
                )
            )
        return Line(segments)


assert Line(
    [Segment(Point(498, 4), Point(498, 6)), Segment(Point(498, 6), Point(496, 6))]
).points_in_between() == [Point(498, 4), Point(498, 5), Point(498, 6), Point(497, 6), Point(496, 6)]


with open("../../data/day14.txt", "r") as f:
    lines = f.readlines()
    lines = [i.strip("\n") for i in lines]
    lines = [Line.new(line.split(" -> ")) for line in lines]
    rocks = list(chain(*[line.points_in_between() for line in lines]))

# Map generation
scene: Dict[Tuple[int, int], str] = {}
max_depth = 0
for rock in rocks:
    scene[(rock.x, rock.y)] = "#"
    max_depth = max(max_depth, rock.y)
# Add floor
# Floor is at max_depth+2
for x in range(0, 1000):
    scene[(x, max_depth + 2)] = "#"
# Sand falls from (500,0)
sand_count = 0
while True:
    sand_pos = Point(500, 0)
    scene[(sand_pos.x, sand_pos.y)] = "o"
    new_sand = False
    sand_count += 1
    while True:
        # For part 1
        # if sand_pos.y == max_depth:
        #     break
        # Move sand down if possible
        bottom = scene.get((sand_pos.x, sand_pos.y + 1))
        if bottom == "#" or bottom == "o":
            pass
        elif bottom is None:
            del scene[(sand_pos.x, sand_pos.y)]
            sand_pos = Point(sand_pos.x, sand_pos.y + 1)
            scene[(sand_pos.x, sand_pos.y)] = "o"
            continue
        # Move sand one step down and to the left
        bottom_left = scene.get((sand_pos.x - 1, sand_pos.y + 1))
        if bottom_left == "#" or bottom_left == "o":
            pass
        elif bottom_left is None:
            del scene[(sand_pos.x, sand_pos.y)]
            sand_pos = Point(sand_pos.x - 1, sand_pos.y + 1)
            scene[(sand_pos.x, sand_pos.y)] = "o"
            continue
        # Move sand one step down and to the right
        bottom_right = scene.get((sand_pos.x + 1, sand_pos.y + 1))
        if bottom_right == "#" or bottom_right == "o":
            pass
        elif bottom_right is None:
            del scene[(sand_pos.x, sand_pos.y)]
            sand_pos = Point(sand_pos.x + 1, sand_pos.y + 1)
            scene[(sand_pos.x, sand_pos.y)] = "o"
            continue
        break
    # Check if sand is at max_depth (For part 1)
    # if sand_pos.y == max_depth:
    #     break
    if sand_pos == Point(500, 0):
        break
    # Print the board
    # for y in range(max_depth + 3):
    #     for x in range(490, 510):
    #         print(scene.get((x, y), "."), end="")
    #     print("\n")
    # print("\n\n")
    # break
# For part 1
print(sand_count - 1)
# For part 2
print(sand_count)
