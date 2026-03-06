import random

# simulates d sampling
def sample_d(gamma, rho, p):
    q_bound = (1 << gamma) // p
    q = random.randint(0, q_bound)
    
    two_rho = 1 << rho
    r_min = -two_rho + 1
    r = random.randint(r_min, two_rho)
    
    return p * q + r

# simulates p n gamma rho sampling
def sample_p(gamma, rho, p, n):
    return [sample_d(gamma, rho, p) for _ in range(n)]

gamma = 6
rho = 2
p = 7
n_val = 8

poly = sample_p(gamma, rho, p, n_val)
assert len(poly) == n_val

two_gamma = 1 << gamma
q_bound = two_gamma // p
two_rho = 1 << rho

x_min = -two_rho + 1
x_max = p * q_bound + two_rho

# tests if all generated polynomial coefficients respect the bounds
for c in poly:
    assert x_min <= c <= x_max

print("test_sample_p passed")