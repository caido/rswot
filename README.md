# RSWOT

[![github](https://img.shields.io/badge/github-caido/rswot.svg?style=for-the-badge&logo=github)](https://github.com/caido/rswot)
[![crates](https://img.shields.io/crates/v/rswot.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/rswot)

This crate helps you identify email addresses that belong to colleges or universities to automate the process of approving or rejecting academic discounts or free tiers.

We use the lists maintained by [Jetbrains](https://github.com/JetBrains/swot).

## Usage

```rust
fn main() {
    let validation = rswot::validate("lreilly@stanford.edu").unwrap();
    println!("Email: {}", validation.email);
    println!("TLD: {}", validation.tld);
    println!("Institution Name: {}", validation.institution_names.unwrap()[0]);
}
```

## Acknowledgements

This project used some code from the original Rust port of [SWOT](https://github.com/orhanbalci/swot).
