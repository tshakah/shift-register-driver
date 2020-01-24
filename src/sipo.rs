//! Serial-in parallel-out shift register

use core::cell::RefCell;
use core::convert::Infallible;
use hal::digital::v2::OutputPin;

macro_rules! ShiftRegisterBuilder {
    ($name: ident, $size: expr) => {
        /// Serial-in parallel-out shift register
        pub struct $name<Pin1, Pin2, Pin3>
        where
            Pin1: OutputPin<Error = Infallible>,
            Pin2: OutputPin<Error = Infallible>,
            Pin3: OutputPin<Error = Infallible>,
        {
            clock: RefCell<Pin1>,
            latch: RefCell<Pin2>,
            data: RefCell<Pin3>,
            output_state: RefCell<[bool; $size]>,
        }

        impl<Pin1, Pin2, Pin3> $name<Pin1, Pin2, Pin3>
        where
            Pin1: OutputPin<Error = Infallible>,
            Pin2: OutputPin<Error = Infallible>,
            Pin3: OutputPin<Error = Infallible>,
        {
            /// Creates a new SIPO shift register from clock, latch, and data output pins
            pub fn new(clock: Pin1, latch: Pin2, data: Pin3) -> Self {
                $name {
                    clock: RefCell::new(clock),
                    latch: RefCell::new(latch),
                    data: RefCell::new(data),
                    output_state: RefCell::new([false; $size]),
                }
            }

            /// Sets the value of the shift register output at `index` to value `command`
            pub fn update(&self, index: usize, command: bool) {
                self.output_state.borrow_mut()[index] = command;
            }

            /// Pushes the current state to the register
            pub fn flush(&self) {
                let output_state = self.output_state.borrow();
                self.latch.borrow_mut().set_low().unwrap();

                for i in 1..=output_state.len() {
                    if output_state[output_state.len() - i] {
                        self.data.borrow_mut().set_high().unwrap();
                    } else {
                        self.data.borrow_mut().set_low().unwrap();
                    }

                    self.clock.borrow_mut().set_high().unwrap();
                    self.clock.borrow_mut().set_low().unwrap();
                }
                self.latch.borrow_mut().set_high().unwrap();
            }

            /// Consume the shift register and return the original clock, latch, and data output pins
            pub fn release(self) -> (Pin1, Pin2, Pin3) {
                let Self {
                    clock,
                    latch,
                    data,
                    output_state: _,
                } = self;
                (clock.into_inner(), latch.into_inner(), data.into_inner())
            }
        }
    };
}

ShiftRegisterBuilder!(ShiftRegister8, 8);
ShiftRegisterBuilder!(ShiftRegister16, 16);
ShiftRegisterBuilder!(ShiftRegister24, 24);
ShiftRegisterBuilder!(ShiftRegister32, 32);
ShiftRegisterBuilder!(ShiftRegister40, 40);
ShiftRegisterBuilder!(ShiftRegister48, 48);
ShiftRegisterBuilder!(ShiftRegister56, 56);
ShiftRegisterBuilder!(ShiftRegister64, 64);
ShiftRegisterBuilder!(ShiftRegister72, 72);
ShiftRegisterBuilder!(ShiftRegister80, 80);
ShiftRegisterBuilder!(ShiftRegister88, 88);
ShiftRegisterBuilder!(ShiftRegister96, 96);
ShiftRegisterBuilder!(ShiftRegister104, 104);
ShiftRegisterBuilder!(ShiftRegister112, 112);
ShiftRegisterBuilder!(ShiftRegister120, 120);
ShiftRegisterBuilder!(ShiftRegister128, 128);

/// 8 output serial-in parallel-out shift register
pub type ShiftRegister<Pin1, Pin2, Pin3> = ShiftRegister8<Pin1, Pin2, Pin3>;
