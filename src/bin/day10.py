from dataclasses import dataclass


@dataclass
class Addx:
    value: int
    run_counter: int

    @staticmethod
    def from_string(string: str):
        return Addx(int(string.split(" ")[1]), 1)

    def do_instruction(self, counter: int):
        return counter + self.value


@dataclass
class Noop:
    run_counter: int

    @staticmethod
    def from_string():
        return Noop(0)

    def do_instruction(self, counter: int):
        return counter


with open("../../data/day10.txt", "r") as f:
    lines = f.readlines()
    lines = [i.strip("\n") for i in lines]
counter = 1
counter_at_cycle = []
instruction_queue = []
screen = []
row = []
for cycle in range(240):

    # Instruction logic
    if len(lines) == 0:
        break
    for queued_instruction in instruction_queue:
        if queued_instruction.run_counter == 0:
            counter = queued_instruction.do_instruction(counter)
            instruction_queue.remove(queued_instruction)
        else:
            queued_instruction.run_counter -= 1
    if len(instruction_queue) == 0:
        inst = lines.pop(0)
        if inst == "noop":
            instruction_queue.append(Noop.from_string())
        elif inst.startswith("addx"):
            instruction = Addx.from_string(inst)
            instruction_queue.append(instruction)
        else:
            raise Exception
        inst_run = inst
    else:
        inst_run = ""
    counter_at_cycle.append(counter)
    # Screen logic
    if len(row) == 40:
        screen.append(row)
        row = []
    sprite_pixels = {counter - 1, counter, counter + 1}
    if cycle % 40 in sprite_pixels:
        row.append("#")
    else:
        row.append(".")
    # print(f"Cycle {cycle+1}: {counter} {inst_run}")
res = sum([counter_at_cycle[i - 1] * i for i in [20, 60, 100, 140, 180, 220]])
print(res)
screen.append(row)
for row in screen:
    print("".join(row))
