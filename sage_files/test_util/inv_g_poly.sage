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

# gadget decomposition for polynomials
def inv_g_poly(coeffs, b, l, q):
    n_val = len(coeffs)
    out = [[0] * n_val for _ in range(l)]
    for i in range(n_val):
        digs = inv_g_zz(coeffs[i], b, l, q)
        for j in range(l):
            out[j][i] = digs[j]
    return out

# tests full reconstruction
input_coeffs = [13, 5, -3, 7]
decomposed = inv_g_poly(input_coeffs, 4, 3, 16)

assert len(decomposed) == 3
for poly in decomposed:
    assert len(poly) == 4

for i in range(4):
    recon = sum(decomposed[j][i] * (4 ** j) for j in range(3))
    assert recon == sym_mod(input_coeffs[i], 16)

# tests processing all coefficients
input_coeffs_2 = [0, 0, 13, 7]
decomposed_2 = inv_g_poly(input_coeffs_2, 4, 3, 16)

any_nonzero = False
for i in range(2, 4):
    for j in range(3):
        if decomposed_2[j][i] != 0:
            any_nonzero = True
assert any_nonzero is True

print("test_inv_g_poly passed")