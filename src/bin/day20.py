from dataclasses import dataclass, field
from itertools import dropwhile, islice
from typing import Iterable, Iterator

with open("../../data/day20.txt", "r") as f:
    lines = f.readlines()
    lines = [i.strip("\n") for i in lines]


@dataclass
class Node:
    val: int
    prev: "Node" = field(init=False)
    next: "Node" = field(init=False)

    @classmethod
    def from_input(cls, it: Iterable[int]) -> "Node":
        node = first = None
        for num in it:
            n = Node(num)
            if node:
                node.link(n)
            else:
                first = n
            node = n
        node.link(first)
        return first

    def __iter__(self) -> Iterator["Node"]:
        node = self
        while True:
            yield node
            node = node.next

    def move(self, list_size: int) -> None:
        i = self.val % (list_size - 1)
        if i:
            p = self
            for _ in range(i):
                p = p.next
            self.prev.link(self.next)
            n = p.next
            p.link(self)
            self.link(n)

    def link(self, n: "Node") -> None:
        self.next = n
        n.prev = self


decryption_key = 811589153
mixing_amount = 10
size = len(lines)
it = Node.from_input(int(line) * decryption_key for line in lines)
pointers = list(islice(it, size))
for _ in range(mixing_amount):
    for node in pointers:
        node.move(size)
a = sum(n.val for n in islice(dropwhile(lambda n: n.val != 0, it), 1000, 3001, 1000))
print(a)
