#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        <M as P<b, y>>::assert();
    }
}

struct M();

// propositions
struct a();
struct b();
struct x();
struct y();

// predicates
trait P<First, Second> { fn assert(); }
trait Q<First> { fn assert(); }

// axioms
impl<Model> P<a, x> for Model { fn assert() {} }

// transitivity rule
impl<Model, First> P<First, y> for Model
where Model: P<First, x>
{ fn assert() {} }

// option: works
// impl P<b, x> for M
// { fn assert() {} }

// option: works
// impl<Model> P<b, x> for Model
// { fn assert() {} }

// option: works
// impl P<b, y> for M
// { fn assert() {} }

// option: doesn't works
// which is weird since there's no P<b, x> to trigger the transitivity rule
// impl<Model> P<b, y> for Model
// { fn assert() {} }

// option: doesn't work
// makes sense in that it would give both P<b, x> and P<b, y>, and hence two ways to P<b, y>
// impl<Second> P<b, Second> for M
// { fn assert() {} }

// option: doesn't work
// makes sense in that it would give both P<b, x> and P<b, y>, and hence two ways to P<b, y>
// impl<Model, Second> P<b, Second> for Model
// { fn assert() {} }

// option: doesn't work
// makes sense since Q is not implemented for M
// impl P<b, y> for M
// where M: Q<b>
// { fn assert() {} }

// option: doesn't work
// which is weird since there's no P<b, x> to trigger the transitivity rule, nor any Q<b>
// impl<Model> P<b, y> for Model
// where Model: Q<b>
// { fn assert() {} }

// option: doesn't work
// which is weird since there's no Q<b>
impl<Model, Second> P<b, Second> for Model
where Model: Q<b>
{ fn assert() {} }
