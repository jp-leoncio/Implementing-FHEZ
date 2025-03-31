## About the project
Implementing FHEZ in Rust for my scientific iniciation in Unicamp.

# TODO list:
- [ ] Double-CRT:
    - [ ] Conversion operations
    - [ ] Addition
    - [ ] Multiplication
        - [ ] DCRT x DCRT -> DCRT
        - [ ] Poly x DCRT -> Poly
    - [ ] Inner Product
        - [ ] ⟨DCRT, DCRT⟩ -> DCRT
        - [ ] ⟨Poly, DCRT⟩ -> Poly
- [ ] Benchmarks:
    - [ ] Operations with DCRT
- [ ] Bibliography:
    - [ ] FHEZ
    - [ ] ZAMA TFHE-rs
    - [X] Double-CRT
    - [X] Concrete-FFT

# References:
[Per21] Hilder Vitor Lima Pereira. Bootstrapping fully homomorphic encryption over the integers in less than one second. Published in [PKC 2021](https://pkc.iacr.org/2021/).