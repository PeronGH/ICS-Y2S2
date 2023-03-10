def main():
    num_names = int(input())
    for _ in range(num_names):
        [last_name, first_name] = input().split()
        print(first_name, last_name)
