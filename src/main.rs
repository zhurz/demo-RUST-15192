use ctrl_macros::{ok_or_return, some_or_return};

struct Test;

impl Test {
    fn hello(&self) {}
}

fn main() {
    println!("Hello, world!");
    let result = Ok((1u8, Test));
    let option = Some((2u8, Test));
    let test = Test;
    let (_, ok_test) = ok_or_return!(result);
    let (_, some_test) = some_or_return!(option);

}

















