#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        <Model as Predicate<b, y>>::assert();
    }
}

struct Model();

trait Predicate<First, Second> {
    fn assert();
}

struct a();
struct b();

struct x();
struct y();

// axiom
impl Predicate<a, x> for Model {
    fn assert() {}
}

// transitivity in second component
impl<First> Predicate<First, y> for Model
where
    Model: Predicate<First, x>
{
    fn assert() {}
}

// option 1: works (on its own)
// impl Predicate<b, x> for Model {
//     fn assert() {}
// }

// option 2: works (on its own)
// impl Predicate<b, y> for Model {
//     fn assert() {}
// }

// option 3: doesn't work (even on its own)
impl<Second> Predicate<b, Second> for Model {
    fn assert() {}
}
