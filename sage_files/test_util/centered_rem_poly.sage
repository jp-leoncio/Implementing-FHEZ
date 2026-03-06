# centered remainder for polynomials
def centered_rem(coeffs, m):
    return [(c % m) - m if (c % m) > m // 2 else (c % m) for c in coeffs]

modulus = 10
poly = [0, 1, 5, 6, -1, -5, -6, 10]
expected = [0, 1, 5, -4, -1, 5, 4, 0]

# tests if polynomial reduces to expected values
result = centered_rem(poly, modulus)
assert result == expected

print("test_centered_rem_poly passed")