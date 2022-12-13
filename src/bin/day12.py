from collections import defaultdict
from dataclasses import dataclass
from typing import Dict, Tuple


@dataclass
class Square:
    value: str

    def can_make_step_to_this(self, other_square: "Square") -> bool:
        """
        Check if this square can be reached from the other square
        """
        assert other_square.value != "E"
        assert self.value != "E"
        if ord(other_square.value) <= (ord(self.value) + 1):
            return True
        else:
            return False

    @property
    def is_start_point(self):
        return self.value == "S"

    @property
    def is_destination(self):
        return self.value == "E"


assert Square("a").can_make_step_to_this(Square("b"))
assert Square("a").can_make_step_to_this(Square("a"))
assert Square("y").can_make_step_to_this(Square("z"))
assert Square("b").can_make_step_to_this(Square("a"))
assert not Square("a").can_make_step_to_this(Square("c"))
with open("../../data/day12.txt", "r") as f:
    lines = f.readlines()
    lines = [i.strip("\n") for i in lines]
    map: Dict[int, Dict[int, Square]] = defaultdict(dict)
    locs_value_a = []
    starting_point_loc = (0, 0)
    destination_loc = (0, 0)
    for i in range(len(lines)):
        for j in range(len(lines[0])):
            map[i][j] = Square(lines[i][j])
            if lines[i][j] == "S":
                starting_point_loc = (i, j)
                map[i][j] = Square("a")
            if lines[i][j] == "E":
                destination_loc = (i, j)
                map[i][j] = Square("z")
            if lines[i][j] == "a" or lines[i][j] == "S":
                locs_value_a.append((i, j))
import heapq as heap


def get_adjacent_nodes(map: Dict[int, Dict[int, Square]], node: Tuple[int, int]) -> list:
    """
    Returns nodes adjacent to node
    """
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    adjacents = []
    for direction in directions:
        new_loc = (node[0] + direction[0], node[1] + direction[1])
        # Check for potential out of bounds
        if square := map.get(new_loc[0], {}).get(new_loc[1]):
            if map[node[0]][node[1]].can_make_step_to_this(square):
                adjacents.append(new_loc)
    return adjacents


def dijkstra(map, starting_node: Tuple[int, int]):
    visited = set()
    parentsMap = {}
    pq = []
    node_cost = defaultdict(lambda: float("inf"))
    node_cost[starting_node] = 0
    heap.heappush(pq, (0, starting_node))
    while pq:
        _, node = heap.heappop(pq)
        visited.add(node)
        for adjNode in get_adjacent_nodes(map, node):
            if adjNode in visited:
                continue
            new_cost = node_cost[node] + 1
            if new_cost < node_cost[adjNode]:
                node_cost[adjNode] = new_cost
                parentsMap[adjNode] = node
                heap.heappush(pq, (new_cost, adjNode))
    return node_cost, parentsMap


tree = dijkstra(map, starting_point_loc)


def get_distance_for_loc(tree, loc: Tuple[int, int]) -> Tuple[int, list]:
    path = []
    while True:
        try:
            parent = tree[1][(loc[0], loc[1])]
        except:
            break
        path.append(parent)
        loc = parent
    return len(path), path


# Part 1
print(get_distance_for_loc(tree, destination_loc)[0])

# Part 2

# Locations with value 'a'
locations = []
for loc in locs_value_a:
    locations.append(get_distance_for_loc(dijkstra(map, loc), destination_loc))
print(min([i[0] for i in locations if i[0] != 0]))
