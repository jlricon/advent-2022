with open("../../data/day6.txt", "r") as f:
    lines: str = f.read()

# Part 1
start_chars = lines[:4]
start_hash = set(lines[:4])
for pos, c in enumerate(lines[4:]):
    if len(start_hash) == 4:
        break
    start_chars = start_chars[1:] + c
    start_hash = set(start_chars)
print(start_chars, pos + 4)


# Part 2
start_chars = lines[:14]
start_hash = set(lines[:14])
for pos, c in enumerate(lines[14:]):
    if len(start_hash) == 14:
        break
    start_chars = start_chars[1:] + c
    start_hash = set(start_chars)
print(start_chars, pos + 14)
