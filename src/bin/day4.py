from dataclasses import dataclass


@dataclass
class Range:
    start: int
    end: int

    @staticmethod
    def from_tuple(tup):
        return Range(int(tup[0]), int(tup[1]))

    def is_included_in_range(self, other: "Range") -> bool:
        return (self.start >= other.start) and (self.end <= other.end)

    def overlaps_other_range(self, other: "Range") -> bool:
        return (self.start <= other.end) and (self.end >= other.start)


@dataclass
class ElfPair:
    first: Range
    second: Range

    @staticmethod
    def from_ranges(ranges):
        return ElfPair(ranges[0], ranges[1])


with open("../../data/day4.txt", "r") as f:
    lines = f.readlines()
    lines = [i.strip("\n") for i in lines]
    lines = [
        ElfPair.from_ranges([Range.from_tuple(j.split("-")) for j in i.split(",")]) for i in lines
    ]
pairs_included = 0
for elf_pair in lines:
    if elf_pair.first.is_included_in_range(elf_pair.second) or elf_pair.second.is_included_in_range(
        elf_pair.first
    ):
        pairs_included += 1
print(pairs_included)
# Count number of pairs that overlap
pairs_overlapping = 0
for elf_pair in lines:
    if elf_pair.first.overlaps_other_range(elf_pair.second):
        pairs_overlapping += 1
print(pairs_overlapping)
