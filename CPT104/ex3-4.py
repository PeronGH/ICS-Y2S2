def main():
    num_fibo = int(input())
    print(fibo(num_fibo+1, [0, 1])[-1])


def fibo(n: int, sequence: list):
    if n == 1:
        return sequence
    else:
        sequence.append(sequence[-1] + sequence[-2])
        return fibo(n-1, sequence)


main()
