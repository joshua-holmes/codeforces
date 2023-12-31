def main():
    num_of_tests = int(input())
    for _ in range(num_of_tests):
        [x, y, n] = [int(n) for n in input().split(" ")]
        multi = (n - y) // x
        answer = (x * multi) + y
        print(answer)

if __name__ == "__main__":
    main()
