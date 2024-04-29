#!/usr/bin/env python3

with open("input.txt", "r") as file:
    text = file.read().strip()

part_1: int = 0
part_2: int | None = None

for n, c in enumerate(text):
    match c:
        case "(":
            part_1 += 1
        case ")":
            part_1 -= 1
        case _:
            print("no")
            exit(1)
    if part_1 == -1 and part_2 is None:
        part_2 = n + 1

print(part_1, part_2)
