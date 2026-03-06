# symmetric reduction of a modulo n
def sym_mod(a, n):
    r = a % n
    return r - n if 2 * r > n else r

# gadget decomposition for a single integer
def inv_g_zz(a, b, l, q):
    copy_val = sym_mod(a, q)
    res = []
    for _ in range(l):
        rem = copy_val % b
        digit = rem - b if 2 * rem > b else rem
        res.append(digit)
        copy_val = (copy_val - digit) // b
    return res

def reconstruct(digits, b):
    return sum(d * (b ** i) for i, d in enumerate(digits))

# tests basic reconstruction
digits_13 = inv_g_zz(13, 4, 3, 16)
assert digits_13 == [1, -1, 0]
assert reconstruct(digits_13, 4) == sym_mod(13, 16)
for d in digits_13:
    assert -2 <= d <= 2

# tests negative value
digits_neg3 = inv_g_zz(-3, 4, 3, 16)
assert reconstruct(digits_neg3, 4) == sym_mod(-3, 16)
for d in digits_neg3:
    assert -2 <= d <= 2

# tests zero
digits_0 = inv_g_zz(0, 4, 3, 16)
assert digits_0 == [0, 0, 0]

# tests various values
for a in [-7, -3, 0, 5, 7, 13, 15, 100]:
    digits = inv_g_zz(a, 4, 3, 16)
    assert reconstruct(digits, 4) == sym_mod(a, 16)
    for d in digits:
        assert -2 <= d <= 2

print("test_inv_g_zz passed")