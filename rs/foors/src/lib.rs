extern crate foo_sys as ffi;

#[derive(Copy, Clone, Debug)]
pub struct Foo {
    value: i32,
}

impl Foo {
    pub fn fill() -> (Foo, i32) {
        let mut foo: std::mem::MaybeUninit<ffi::FOO> = std::mem::MaybeUninit::uninit();
        let foo = foo.as_mut_ptr();

        let res = unsafe { ffi::FooFill(foo) };
        let foo = unsafe { *foo };
        (Foo { value: foo.Value }, res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (f, r) = Foo::fill();
        println!("{:?} {}", f, r);
    }
}
