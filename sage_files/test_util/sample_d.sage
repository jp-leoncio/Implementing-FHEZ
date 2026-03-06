import random

# simulates d sampling
def sample_d(gamma, rho, p):
    q_bound = (1 << gamma) // p
    q = random.randint(0, q_bound)
    
    two_rho = 1 << rho
    r_min = -two_rho + 1
    r = random.randint(r_min, two_rho)
    
    return p * q + r

gamma = 6
rho = 2
p = 7

two_gamma = 1 << gamma
q_bound = two_gamma // p
two_rho = 1 << rho

x_min = -two_rho + 1
x_max = p * q_bound + two_rho

# tests if random samples respect the bounds
for _ in range(1000):
    x = sample_d(gamma, rho, p)
    assert x_min <= x <= x_max

print("test_sample_d passed")