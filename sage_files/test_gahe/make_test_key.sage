# checks parameters that satisfy the correctness condition
p = 19
t = 3
rho = 1
x0 = 38003

alpha = round(p / t)
assert alpha == 6
assert 2**rho + (t - 1) * 0.5 < p / (2 * t)

# validates modular inverse
k_inv_0 = pow(3, -1, x0)
assert k_inv_0 == 12668
assert (3 * 12668) % 38003 == 1

print("test_make_test_key passed")