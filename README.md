# Benchmarking different implementations of Ed25519 Generate Keys / Sign / Verify

The testing has been performed on Windows 7x64 host with Ubuntu 18.04 x64 virtual machine with 8Gb RAM and 4 cores, Intel-VT enabled.

Host:
- CPU: Intel Core i7-4790 3.6 GHz 4 cores / 8 threads
- RAM: 32 Gb

AVX2 is used whenever possible.

Full release optimizations.

Ed25519 is used over 64 byte hash with 100'000 cycles.

All tests use 1 core.

Test results are given in milliseconds of total execution time (generating/signing/verifying), in descending order, worst to best.

You can find links to the algorithms' web pages in the first comments of the corresponding main source files.

## Test Results with SHA-256

| Language | Author  | Time, ms  |
|:-------:|:---------:|:---------:|
|  rust  | openssl   | 4318/4218/11302 |
|  rust  | DaGenix   | 3824/3971/12805 |
|  rust  | dalek   | 2194/2277/6012 |
