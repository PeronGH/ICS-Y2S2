def main():
    num_cars = int(input())
    car_weights: list[float] = []

    for _ in range(num_cars):
        car_weights.append(float(input()))

    avg_weight = sum(car_weights) / num_cars

    for weight in car_weights:
        print(avg_weight - weight)
