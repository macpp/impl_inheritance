#![feature(specialization)]

#[macro_use]
extern crate impl_inheritance;

#[test]
fn ass_type() {
    use std::mem::size_of;
    let ch1 = Child1Data::default();
    assert_eq!(ch1.foo(),0);
    assert_eq!(size_of::< < BaseData as Base> ::AssType>(), 4);
    assert_eq!(size_of::< < Child1Data as Base> ::AssType>(), 8);
    assert_eq!(size_of::< < Child2Data as Base> ::AssType>(), 8);

}

#[test]
fn ass_const(){
    assert_eq!( <BaseData as Base> :: ASS_CONST, 0);
    assert_eq!( <Child1Data as Base> :: ASS_CONST, 1);
    assert_eq!( <Child2Data as Base> :: ASS_CONST, 1);
}

//--level_0--

#[inheritable(BaseData)]
trait Base {
    fn foo(&self) -> i32;
    type AssType;
    const ASS_CONST: i8;
}

#[derive(Default,Debug,Base)]
struct BaseData {
    x: i32
}

impl Base for BaseData {
    fn foo(&self) -> i32 {
         0
    }
    type AssType = i32;
    const ASS_CONST: i8 = 0;
}

//--level_1--

#[inheritable(Child1Data)]
trait Child1 : Base {
    fn foo1(&self);
}

#[derive(Default,Debug,Inherites)]
struct Child1Data {
    x1 : i32,
    #[super_data]
    super_data: BaseData,
}

impl Child1 for Child1Data {
    fn foo1(&self) {
        println!("foo1 Child1Data: {}", self.x1);
    }
}

#[overrides]
impl Base for Child1Data {
    type AssType = i64;
    const ASS_CONST: i8 = 1;
}

//--level_2--

#[inheritable(Child2Data)]
trait Child2 : Child1 {
    fn foo2(&self) -> i32;
}


#[derive(Default,Debug,Inherites)]
struct Child2Data {
    x2 : i32,
    #[super_data]
    super_data: Child1Data,
}

impl Child2 for Child2Data {
    fn foo2(&self) -> i32 {
        2002
    }
}