# ip-filter

## Observe this output

```bash
❯ llvm-readelf --sections target/debug/build/ip-filter-0cfcb0e2f15b2963/out/ip-filter
There are 23 section headers, starting at offset 0x1a358:

Section Headers:
  [Nr] Name              Type            Address          Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            0000000000000000 000000 000000 00      0   0  0
  [ 1] .strtab           STRTAB          0000000000000000 01a1c8 00018d 00      0   0  1
  [ 2] .text             PROGBITS        0000000000000000 000040 000260 00  AX  0   0  8
  [ 3] xdp               PROGBITS        0000000000000000 0002a0 0006a8 00  AX  0   0  8
  [ 4] .relxdp           REL             0000000000000000 010458 000050 10   I 22   3  8
  [ 5] license           PROGBITS        0000000000000000 000948 00000d 00   A  0   0  1
  [ 6] .rodata           PROGBITS        0000000000000000 000955 000038 00   A  0   0  1
  [ 7] maps              PROGBITS        0000000000000000 000990 000038 00  WA  0   0  4
  [ 8] .debug_loc        PROGBITS        0000000000000000 0009c8 002255 00      0   0  1
  [ 9] .debug_abbrev     PROGBITS        0000000000000000 002c1d 000515 00      0   0  1
  [10] .debug_info       PROGBITS        0000000000000000 003132 0051b3 00      0   0  1
  [11] .rel.debug_info   REL             0000000000000000 0104a8 0095f0 10   I 22  10  8
  [12] .debug_ranges     PROGBITS        0000000000000000 0082e5 0003f0 00      0   0  1
  [13] .debug_str        PROGBITS        0000000000000000 0086d5 0054d7 01  MS  0   0  1
  [14] .BTF              PROGBITS        0000000000000000 00dbac 000e31 00      0   0  4
  [15] .rel.BTF          REL             0000000000000000 019a98 000030 10   I 22  14  8
  [16] .BTF.ext          PROGBITS        0000000000000000 00e9e0 000618 00      0   0  4
  [17] .rel.BTF.ext      REL             0000000000000000 019ac8 000600 10   I 22  16  8
  [18] .debug_frame      PROGBITS        0000000000000000 00eff8 0000a0 00      0   0  8
  [19] .rel.debug_frame  REL             0000000000000000 01a0c8 0000c0 10   I 22  18  8
  [20] .debug_line       PROGBITS        0000000000000000 00f098 00117b 00      0   0  1
  [21] .rel.debug_line   REL             0000000000000000 01a188 000040 10   I 22  20  8
  [22] .symtab           SYMTAB          0000000000000000 010218 000240 18      1  15  8
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  R (retain), p (processor specific)

❯ llvm-objdump --no-show-raw-insn --section=xdp -S target/debug/build/ip-filter-0cfcb0e2f15b2963/out/ip-filter

```

## Prerequisites

1. stable rust toolchains: `rustup toolchain install stable`
1. nightly rust toolchains: `rustup toolchain install nightly --component rust-src`
1. (if cross-compiling) rustup target: `rustup target add ${ARCH}-unknown-linux-musl`
1. (if cross-compiling) LLVM: (e.g.) `brew install llvm` (on macOS)
1. (if cross-compiling) C toolchain: (e.g.) [`brew install filosottile/musl-cross/musl-cross`](https://github.com/FiloSottile/homebrew-musl-cross) (on macOS)
1. bpf-linker: `cargo install bpf-linker` (`--no-default-features` on macOS)

## Build & Run

Use `cargo build`, `cargo check`, etc. as normal. Run your program with:

```shell
cargo run --release --config 'target."cfg(all())".runner="sudo -E"'
```

Cargo build scripts are used to automatically build the eBPF correctly and include it in the
program.

## Cross-compiling on macOS

Cross compilation should work on both Intel and Apple Silicon Macs.

```shell
CC=${ARCH}-linux-musl-gcc cargo build --package ip-filter --release \
  --target=${ARCH}-unknown-linux-musl \
  --config=target.${ARCH}-unknown-linux-musl.linker=\"${ARCH}-linux-musl-gcc\"
```
The cross-compiled program `target/${ARCH}-unknown-linux-musl/release/ip-filter` can be
copied to a Linux server or VM and run there.

## License

With the exception of eBPF code, ip-filter is distributed under the terms
of either the [MIT license] or the [Apache License] (version 2.0), at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

### eBPF

All eBPF code is distributed under either the terms of the
[GNU General Public License, Version 2] or the [MIT license], at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the GPL-2 license, shall be
dual licensed as above, without any additional terms or conditions.

[Apache license]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
[GNU General Public License, Version 2]: LICENSE-GPL2
