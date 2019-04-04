#![feature(specialization)]

#[macro_use]
extern crate impl_inheritance;

#[test]
pub fn mod_main(){
    //use inherit::SuperBorrow;
    println!("ch1");
    let ch1 = Child1Data::default();
    assert_eq!(ch1.foo(),0);

    //let _y : & BaseData = ch1.super_ref();

    println!("ch2");
    let ch2 = Child2Data::default();
    assert_eq!(ch2.foo(),2);
    ch2.foo1();
    assert_eq!(ch2.foo2(),2002);

    println!("ch3a");
    let ch3a = Child3aData::default();
    assert_eq!(ch3a.foo(),2);
    ch3a.foo1();
    assert_eq!(ch3a.foo2(),2002);
    ch3a.foo3a();

    println!("ch3b");
    let ch3b = Child3bData::default();
    assert_eq!(ch3b.foo(),2);
    ch3b.foo1();
    assert_eq!(ch3b.foo2(),2003);
    ch3b.foo3b();

    println!("ch4");
    let mut ch4 = Child4Data::default();
    assert_eq!(ch4.foo(),4);
    ch4.foo1();
    assert_eq!(ch4.foo2(),2003);
    ch4.foo3b();
    assert_eq!(ch4.foo4(),2);
    assert_eq!(ch4.foo4(),6);
    assert_eq!(ch4.foo4(),14);

    println!("ch5");
    let mut ch5 = Child5Data::default();
    assert_eq!(ch5.foo(),4);
    ch5.foo1();
    assert_eq!(ch5.foo2(),2003);
    ch5.foo3b();
    ch5.foo4();
    ch5.foo5();
}

//--level_0--

#[inheritable(BaseData)]
trait Base {
    fn foo(&self) -> i32;
}

#[derive(Default,Debug,Base)]
struct BaseData {
    x: i32
}

impl Base for BaseData {
    fn foo(&self) -> i32 {
         println!("foo BaseData: {}", self.x);
         0
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
        println!("foo2 Child2Data: {}", self.x2);
        2002
    }
}

//overriding 

#[overrides]
impl Base for Child2Data {
    fn foo(&self) -> i32 {
         println!("foo Child2Data!!!!!!!!!");
         2
    }
}


//--level_3--

//a 
#[inheritable(Child3aData)]
trait Child3a : Child2 {
    fn foo3a(&self);
}

#[derive(Default,Debug,Inherites)]
struct Child3aData {
    x3 : i32,
    #[super_data]
    super_data: Child2Data,
}


impl Child3a for Child3aData {
    fn foo3a(&self) {
        println!("foo3a Child3aData: {}", self.x3);
    }
}

//b

#[inheritable(Child3bData)]
trait Child3b : Child2 {
    fn foo3b(&self);
}

#[derive(Default,Debug,Inherites)]
struct Child3bData {
    x3 : i32,
    #[super_data]
    super_data: Child2Data,
}

impl Child3b for Child3bData {
    fn foo3b(&self) {
        println!("foo3b Child3bData: {}", self.x3);
    }
}

#[overrides]
impl Child2 for Child3bData {
    fn foo2(&self) -> i32 {
        println!("foo2 Child3Data: {}%%%", self.x3);
        2003
    }
}

//--level_4--


#[inheritable(Child4Data)]
trait Child4 : Child3b {
    fn foo4(& mut self) -> i32;
}

#[derive(Default,Debug,Inherites)]
struct Child4Data {
    x4 : i32,
    #[super_data]
    super_data: Child3bData,
}

impl Child4 for Child4Data {
    fn foo4(& mut self) -> i32 {
        self.x4 += 1;
        self.x4 *=2;
        println!("foo4 Child4Data: {}", self.x4);
        self.x4
    }
}

#[overrides]
impl Base for Child4Data {
    fn foo(&self) -> i32 {
         println!("foo Child4Data!!!!!!!!!%%%%%%%%%%%%%");
         4
    }
}

//level_5--


#[inheritable(Child5Data)]
trait Child5 : Child4 {
    fn foo5(& mut self);
}

#[derive(Default,Debug,Inherites)]
struct Child5Data {
    x5 : i32,
    #[super_data]
    super_data: Child4Data,
}

impl Child5 for Child5Data {
    fn foo5(& mut self) {
        self.x5 += 1;
        println!("foo4 Child4Data: {}", self.x5);
    }
}
