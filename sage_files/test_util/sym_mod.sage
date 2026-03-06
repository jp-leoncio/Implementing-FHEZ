# symmetric reduction of a modulo n
def sym_mod(a, n):
    r = a % n
    return r - n if 2 * r > n else r

assert sym_mod(7, 5) == 2
assert sym_mod(-7, 5) == -2
assert sym_mod(-3, 5) == 2
assert sym_mod(3, 5) == -2
assert sym_mod(0, 5) == 0
assert sym_mod(10, 5) == 0
assert sym_mod(-10, 5) == 0
assert sym_mod(6, 12) == 6
assert sym_mod(-6, 12) == 6
print("test_sym_mod passed")