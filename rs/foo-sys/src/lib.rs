#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_all_symbols() {
        let mut foo: std::mem::MaybeUninit<FOO> = std::mem::MaybeUninit::uninit();
        let foo = foo.as_mut_ptr();

        let res = unsafe { FooFill(foo) };
        let foo = unsafe { *foo };
        println!("result: {}", res);
        println!("Value: {}", foo.Value);
    }
}
