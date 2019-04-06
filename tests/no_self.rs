#![feature(specialization)]

#[macro_use]
extern crate impl_inheritance;

#[test]
fn base(){
    assert_eq!(BaseData::foo(),0);
}

#[test]
fn child1(){
    assert_eq!(Child1Data::foo(),1);
    assert_eq!(Child1Data::foo1(3), 6);
}

#[test]
fn child2(){
    assert_eq!(Child2Data::foo(),1);
    assert_eq!(Child2Data::foo1(3), 6);
    assert_eq!(Child2Data::foo2(3), 9);
}

//--level_0--

#[inheritable(BaseData)]
trait Base {
    fn foo() -> i32;
}

#[derive(Default,Debug,Base)]
struct BaseData {
    x: i32
}

impl Base for BaseData {
    fn foo() -> i32 {
         0
    }
}

//--level_1--

#[inheritable(Child1Data)]
trait Child1 : Base {
    fn foo1(a : i32) -> i32;
}

#[derive(Default,Debug,Inherites)]
struct Child1Data {
    x1 : i32,
    #[super_data]
    super_data: BaseData,
}

impl Child1 for Child1Data {
    fn foo1(a : i32) -> i32 {
        a *2
    }
}

#[overrides]
impl Base for Child1Data {
    fn foo() -> i32 {
        1
    }
}

//--level_2--

#[inheritable(Child2Data)]
trait Child2 : Child1 {
    fn foo2(b: i32) -> i32;
}


#[derive(Default,Debug,Inherites)]
struct Child2Data {
    x2 : i32,
    #[super_data]
    super_data: Child1Data,
}

impl Child2 for Child2Data {
    fn foo2(b: i32) -> i32 {
        b * 3
    }
}