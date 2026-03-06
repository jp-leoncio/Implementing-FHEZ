# signed decomposition in base b without modular reduction
def decomp(val, b, l):
    res = []
    cur = val
    for _ in range(l):
        rem = cur % b
        d = rem - b if 2 * rem > b else rem
        res.append(d)
        cur = (cur - d) // b
    return res

def reconstruct(digits, b):
    return sum(d * (b ** i) for i, d in enumerate(digits))

# tests multiple values for correct reconstruction
for val in [0, 1, 13, -13, 100, -100, 255, -255]:
    digits = decomp(val, 4, 6)
    assert len(digits) == 6
    assert reconstruct(digits, 4) == val
    for d in digits:
        assert -2 <= d <= 2

print("test_signed_base_b_decomposition passed")