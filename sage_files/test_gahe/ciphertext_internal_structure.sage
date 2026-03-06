# helper functions
def poly_mul_mod(a, b, n, x0):
    result = [0] * n
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            idx = (i + j) % n
            sign = -1 if ((i + j) // n) % 2 == 1 else 1
            result[idx] = (result[idx] + sign * ai * bj) % x0
    return result

# checks structural layout of the ciphertext components
p = 19
t = 3
n = 4
x0 = 38003
alpha = 6
k = [3, 0, 0, 0]
k_inv = [12668, 0, 0, 0]

m = [2, 0, 1, 1]
q = [1, 0, 2, 0]
r = [0, 1, -1, 0]

pq_r_m = [(p * q[i] + r[i] + m[i] * alpha) % x0 for i in range(n)]

c = poly_mul_mod(pq_r_m, k, n, x0)
cp = poly_mul_mod(c, k_inv, n, x0)

assert cp == pq_r_m

print("test_ciphertext_internal_structure passed")