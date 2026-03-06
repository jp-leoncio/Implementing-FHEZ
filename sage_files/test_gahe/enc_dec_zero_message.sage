# helper functions
def poly_mul_mod(a, b, n, x0):
    result = [0] * n
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            idx = (i + j) % n
            sign = -1 if ((i + j) // n) % 2 == 1 else 1
            result[idx] = (result[idx] + sign * ai * bj) % x0
    return result

def centered_rem(v, m):
    r = v % m
    return r - m if r > m // 2 else r

def round_div(num, den):
    import math
    return math.floor((num + den / 2) / den)

# tests encryption and decryption of a zero message
p = 19
t = 3
n = 4
x0 = 38003
alpha = 6
k = [3, 0, 0, 0]
k_inv = [12668, 0, 0, 0]

m = [0, 0, 0, 0]
q = [3, 0, 2, 1]
r = [-1, 1, 0, -1]

pq_r_m = [p * q[i] + r[i] + m[i] * alpha for i in range(n)]
c = poly_mul_mod(pq_r_m, k, n, x0)
cp = poly_mul_mod(c, k_inv, n, x0)

dec = [round_div(t * centered_rem(ci, p), p) % t for ci in cp]
assert dec == m

print("test_encrypt_decrypt_zero_message passed")