use bar::Unused as _;
use core::fmt::Debug;

fn _foo<Error: Debug>() {
    todo!()
}

pub fn foo() {
    _foo();
}
