# Orderbook
## Description 
This project is an implementation of an order book in Rust. The order book is stored using two [B-tree](https://en.wikipedia.org/wiki/B-tree) data structures (represented by [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) in rust), which guarantee O(log n) time complexity for key operations according to the wiki.  
## Installing dependencies and starting the program
### Linux/Mac os
1. Install rust if not installed 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Start the program
```bash
cargo run
```
### Nix/Nixos
1. Start the program

```bash
nix run "place holder"
```


