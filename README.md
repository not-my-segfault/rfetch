# rfetch
cross platform fetch tool written in rust

---

![](screenshot.png)

---
### compilation / installation:
to compile for linux:
- `make clean linux`

to compile for windows:
- `make clean win`

to test:
- `make test`

### dependencies:
linux:
- rustup (that's it!)

windows:
- rustup
- the `x86_64-pc-windows-gnu` rustup toolchain 

  (to install: `rustup target add x86_64-pc-windows-gnu`)
