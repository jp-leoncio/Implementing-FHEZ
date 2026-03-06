import random

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

# loop through random values to ensure stability
p = 19
t = 3
n = 4
x0 = 38003
rho = 1
gamma = 10
alpha = 6
q_bound = (1 << gamma) // p

k = [3, 0, 0, 0]
k_inv = [12668, 0, 0, 0]

for _ in range(50):
    m = [random.randint(0, t - 1) for _ in range(n)]
    q = [random.randint(0, q_bound) for _ in range(n)]
    r = [random.randint(-(2**rho) + 1, 2**rho) for _ in range(n)]
    
    pq_r_m = [p * q[i] + r[i] + m[i] * alpha for i in range(n)]
    c = poly_mul_mod(pq_r_m, k, n, x0)
    cp = poly_mul_mod(c, k_inv, n, x0)
    
    dec = [round_div(t * centered_rem(ci, p), p) % t for ci in cp]
    assert dec == m
    
print("test_random_encrypt_decrypt_multiple_rounds passed")