with open("../../data/day2.txt", "r") as f:
    lines = f.readlines()
    lines = [i.strip("\n") for i in lines]


def shape_value(shape: str) -> int:
    match shape:
        case "X" | "A":
            return 1
        case "Y" | "B":
            return 2
        case "Z" | "C":
            return 3
        case _:
            raise Exception


def who_wins(my_shape, their_shape) -> int:
    match (my_shape, their_shape):
        case ("X", "A") | ("Y", "B") | ("Z", "C"):
            return 3
        case ("Y", "A") | ("Z", "B") | ("X", "C"):
            return 6
        case _:
            return 0


assert who_wins("Y", "A") == 6


def round_to_score(round: str) -> int:
    their_shape, my_shape = round.split(" ")
    my_shape_score = shape_value(my_shape)
    win_score = who_wins(my_shape, their_shape)
    return my_shape_score + win_score


res = sum(map(round_to_score, lines))
print(res)
# Part 2
def get_my_shape(their_shape: str, my_instruction: str) -> str:
    """
    their_shape is A,B, or C. my_instruction is X,Y, or Z
    A represents rock, B is paper, C is scissors.
    X means lose, Y means draw, Z means win
    """
    print(their_shape, my_instruction)
    match (their_shape, my_instruction):
        case ("A", "X") | ("B", "Z") | ("C", "Y"):
            return "Z"
        case ("A", "Y") | ("B", "X") | ("C", "Z"):
            return "X"
        case ("A", "Z") | ("B", "Y") | ("C", "X"):
            return "Y"
        case _:
            raise Exception


assert get_my_shape("A", "Y") == "X"
assert get_my_shape("B", "X") == "X"
assert get_my_shape("C", "Z") == "X"


def part2(round: str) -> int:
    their_shape, my_instruction = round.split(" ")
    my_shape = get_my_shape(their_shape, my_instruction)
    my_shape_score = shape_value(my_shape)
    win_score = who_wins(my_shape, their_shape)
    print(my_shape_score + win_score)
    return my_shape_score + win_score


res = sum(map(part2, lines))
print(res)
