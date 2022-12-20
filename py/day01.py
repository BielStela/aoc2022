example = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""


def part1(input):
    parts = input.split("\n")
    calories = 0
    res = []
    for val in parts:
        if val:
            calories += int(val)
        else:
            res.append(calories)
            calories = 0
    return res


if __name__ == "__main__":
    assert max(part1(example)) == 24000
    with open("../data/1.txt") as f:
        res = part1(f.read())
        print(max(res))
        print(sum(list(sorted(res, reverse=True))[:3]))

