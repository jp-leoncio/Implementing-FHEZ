## About the project
Implementing FHEZ in Rust for my scientific iniciation in Unicamp.

# TODO list:
- [ ] Double-CRT:
    - [X] Conversion operations
    - [X] Addition
    - [X] Multiplication
        - [X] DCRT x DCRT -> DCRT
        - [X] DCRT x DCRT -> Poly
        - [X] Poly x DCRT -> Poly
    - [X] Inner Product
        - [X] ⟨DCRT, DCRT⟩ -> DCRT
        - [X] ⟨Poly, DCRT⟩ -> DCRT
        - [X] ⟨Poly, Poly⟩ -> DCRT
        - [X] ⟨DCRT, DCRT⟩ -> Poly
    - [ ] Evaluation
- [ ] Benchmarks:
    - [ ] Operations with DCRT
- [ ] Bibliography:
    - [ ] FHEZ
    - [ ] ZAMA TFHE-rs
    - [X] Double-CRT
    - [X] Concrete-FFT

# References:
[Per21] Hilder Vitor Lima Pereira. Bootstrapping fully homomorphic encryption over the integers in less than one second. Published in [PKC 2021](https://pkc.iacr.org/2021/).