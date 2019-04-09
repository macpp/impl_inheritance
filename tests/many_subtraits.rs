#![feature(specialization)]

#[macro_use]
extern crate impl_inheritance;
use std::fmt::Debug;

#[test]
fn child1(){
    let ch1 = Child1Data::default();
    assert_eq!(ch1.foo(),1);
    ch1.print_foo();
}

#[test]
fn child2(){
    let ch2 = Child2Data::default();
    assert_eq!(ch2.foo(),1);
    ch2.foo1();
    assert_eq!(ch2.foo2(),2002);
    ch2.print_foo();
}

//--level_0--

#[inheritable_for(BaseData)]
trait Base : Debug + Default{
    fn foo(&self) -> i32;
}

#[derive(Default,Debug,Base)]
struct BaseData {
    x: i32
}

impl Base for BaseData {
    fn foo(&self) -> i32 {
         0
    }
}

//--level_1--

#[inheritable_for(Child1Data)]
trait Child1 : Base + FooPrinter {
    fn foo1(&self);
}

#[derive(Default,Debug,InheritesImpls)]
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

impl FooPrinter for Child1Data {

}

//overriding 

#[overrides]
impl Base for Child1Data {
    fn foo(&self) -> i32 {
         1
    }
}


//--level_2--


#[inheritable_for(Child2Data)]
trait Child2 : Child1 {
    fn foo2(&self) -> i32;
}


#[derive(Default,Debug,InheritesImpls)]
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

impl FooPrinter for Child2Data{

}


trait FooPrinter {
    fn print_foo(&self) {
        println!("Foo");
    }
}
