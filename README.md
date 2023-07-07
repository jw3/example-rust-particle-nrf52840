Rust on Particle gen 3 boards (nrf52840)
===

Bootstrap dev with Rust on Particle Xenon / Argon / Boron

## requirements
1. [2nd gen debugger](https://github.com/particle-iot/debugger)
2. [probe-run](https://docs.rs/crate/probe-run/latest)
3. [flip-link](https://docs.rs/crate/flip-link/latest)
4. thumbv7em-none-eabihf

## steps
1. connect debugger to xenon, insert to usb
2. export `DEFMT_LOG=info` (to see output from main.rs)
3. `cargo run --example hello`

## notes
1. Scaffold future projects with https://github.com/knurling-rs/app-template
2. the debugger was not initially found by `probe-run --list-probes`
   - holding in the button while inserting it and it worked
   - after that even it works without holding button

## references
- https://github.com/knurling-rs
- https://github.com/daschl/nrf52840dk-sample
- https://github.com/nrf-rs/nrf-hal
- 

## troubleshooting
- https://ferrous-systems.com/blog/defmt-rtt-linker-error/
- 
