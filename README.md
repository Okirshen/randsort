# Randsort
[![docs.rs](https://docs.rs/randsort/badge.svg)](https://docs.rs/randsort)
[![Crates.io](https://img.shields.io/crates/v/randsort.svg)](https://crates.io/crates/randsort)

A sorting library with an optimal complexity of O(n) (!!) by randomly sorting Vectors until they are sorted.

## Sorting Made Easy
Over the years I have discovered many sorting algorithms. Some are bad, some are worse. After searching for years for the perfect algorithms I have decided to create it myself.
No more learning complex algorithms. No more benchmarking millisecond differences.

With Randsort the rand create handles everything for you by scrambling your Vec until it sorts itself out.

## Usage
In order to use the randsort algorithm simply run the .randsort function on your Vec<T> where T implements PartialOrd. Like so:
```rust
let mut vec = vec![5, 4, 8, 9, 12, 1, 3909, 567, 5, 6];
vec.randsort(); // => [1, 4, 5, 5, 6, 8, 9, 12, 567, 3909]

```

## License
This crate is licensed under the [MIT license](https://github.com/Okirshen/randsort/blob/master/LICENSE) (GPL isn't really libre).
