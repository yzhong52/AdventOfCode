def speak(until: int, debug: bool):
    starting_numbers = [7, 12, 1, 0, 16, 2]

    previously_spoken = {}
    spoken = {n: i for i, n in enumerate(starting_numbers)}

    last_spoken = starting_numbers[-1]

    for i in range(len(starting_numbers), until):
        if last_spoken in previously_spoken:
            last_spoken = spoken[last_spoken] - previously_spoken[last_spoken]
        else:
            last_spoken = 0

        if last_spoken in spoken:
            previously_spoken[last_spoken] = spoken[last_spoken]
        spoken[last_spoken] = i

        if debug:
            print(f"Turn {i + 1}, speak {last_spoken}")
    return last_spoken


print(speak(2020, debug=True))
print(speak(30000000, debug=False))
