#![feature(specialization)]

#[test]
fn self_by_value() {
    let base = BaseData{x: 1};
    assert_eq!(base.foo(), 1);

    let child1 = Child1Data{x1 : 2, super_data: BaseData{x: 1}};
    assert_eq!(child1.foo(), 2);

    let child2 = Child2Data{x2: 4, super_data: Child1Data{x1 : 2, super_data: BaseData{x: 1}}};
    assert_eq!(child2.foo(), 2);
}

#[test]

fn struct_with_drop() {
    
}

#[macro_use]
extern crate impl_inheritance;

//--level_0--

#[inheritable(BaseData)]
trait Base {
    fn foo(self) -> i32;
}

#[derive(Default,Debug,Base)]
struct BaseData {
    x: i32
}

impl Base for BaseData {
    fn foo(self) -> i32 {
         self.x
    }
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
    fn foo(self) -> i32 {
        self.x1
    }
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