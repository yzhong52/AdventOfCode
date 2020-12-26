def guess_loop_size(target_public_key: int) -> int:
    subject_number = 7
    value = 1
    loop_size = 0
    while value != target_public_key:
        loop_size += 1
        value = value * subject_number % 20201227
    return loop_size


def calculate_encryption_key(public_key: int, loop_size: int):
    value = 1
    for _ in range(loop_size):
        value = value * public_key % 20201227
    return value


door_public_key = 12092626
card_public_key = 4707356
door_loop_size = guess_loop_size(door_public_key)
card_loop_size = guess_loop_size(card_public_key)
encryption_key1 = calculate_encryption_key(card_public_key, door_loop_size)
encryption_key2 = calculate_encryption_key(door_public_key, card_loop_size)
print(encryption_key1, encryption_key2)
