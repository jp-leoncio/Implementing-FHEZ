# --- Script Sage para Verificação ---
N = 8
P.<x> = PolynomialRing(ZZ)
R = P.quotient(x^N + 1, 'x')
poly_a = R("(-3092145652) + (2408566850) * x + (3409297708) * x^2 + (3139393399) * x^3 + (793343409) * x^4 + (-3288478694) * x^5 + (-1928678263) * x^6 + (-3960511912) * x^7")
poly_b = R("(-2557721838) + (-1405563542) * x + (78269998) * x^2 + (4187384960) * x^3 + (1571914859) * x^4 + (1358335713) * x^5 + (3358304898) * x^6 + (3612871878) * x^7")
sage_sum = poly_a + poly_b
sage_mul = poly_a * poly_b
print("[SAGE_SUM]")
print(sage_sum)
print("[SAGE_MUL]")
print(sage_mul)
