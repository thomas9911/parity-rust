
# Parity

Small rust library to calculate the parity (if a number is even or odd) of a (large) number.

```rust
fn usage() {
    // find the parity of a large number

    // (845**123 + 12) ** (547 * 845**814)

    let result = power(add(power(854, 123), 12), multiply(power(845, 814), 547));
    assert_eq!(result, true);

    // (547 * 845**81254) ** (8541**12345 + 123)

    let result = power(
        multiply(power(845, 81254), 547),
        add(power(8541, 12345), 123),
    );
    assert_eq!(result, false);
}

fn usage_on_function() {
    // x*x + 2
    let func = |x: u64| add(multiply(x, x), 2u8);

    let n0 = 0..;

    // 0, 3, 6, 11
    let first_4: Vec<_> = n0.map(func).take(4).collect();

    assert_eq!(vec![true, false, true, false], first_4);
}
```
