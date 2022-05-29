def solve(num: int) -> int:
    sum = 0
    for i in range(1, num):
        if i % 3 == 0 or i % 5 == 0:
            sum += i
    return sum


def test_problem001():
    assert solve(10) == 23
    assert solve(20) == 78
    assert solve(1000) == 233168


if __name__ == "__main__":
    print(solve(1000))
