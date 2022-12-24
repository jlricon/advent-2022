import numpy as np

with open("../../data/day20.txt", "r") as f:
    lines = f.readlines()
    lines = np.array([int(i.strip("\n")) for i in lines])
# assert len(lines) == len(set(lines))


def mix(numbers):
    numbers2 = np.array(numbers)
    for i, num in enumerate(numbers):
        numbers2 = np.roll(numbers, -num)
        numbers2[-num] = num

        print(numbers2)
    return numbers2


def find_grove_coordinates(numbers):
    mixed = mix(numbers)
    zero_index = mixed.tolist().index(0)
    coord1 = mixed[(zero_index + 1000) % len(mixed)]
    coord2 = mixed[(zero_index + 2000) % len(mixed)]
    coord3 = mixed[(zero_index + 3000) % len(mixed)]
    return coord1 + coord2 + coord3


result = find_grove_coordinates(lines)
print(result)  # Expected output: 3
