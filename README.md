# shift-register-driver

> Platform agnostic driver for shift register's built using the embedded-hal. 

## What works

- Controlling outputs through serial-in parallel-out shift registers with 8 outputs

## TODO

- [ ] Update naming and module structure to make room for adding parallel-out serial-in shift register support
- [ ] Add parallel-out serial-in shift register support
- [ ] Support chained shift registers for more than 8 IO

## Examples

```rust
let shift_register = ShiftRegister::new(clock, latch, data);
let mut outputs = shift_register.decompose();

loop {

    for i in 0..8 {
        // Optionally control the pins directly through the shift_register struct
        //      or individually by using the decompose method
        // shift_register.update(i, false);
        // shift_register.update((i+1)%4, true);

        outputs[i].set_low();
        outputs[(i+1)%8].set_high();

        delay.delay_ms(300u32);
    }

}
```
    
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.