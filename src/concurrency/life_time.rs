//lifetime elision rules
// 1- if there is only one input lifetime it is assigned to all output lifetimes
// 2- if there is 'self' lifetime it is assigned to all output lifetimes

// it doesn't mean that lifetimes are the same
// it means that:
// 1- there is a relationship between the parameters and the return value
// 2- the relationship is that; return value could be the shortest of the input parameters
fn find_longest<'a>(first:&'a str, second:&'a str)->&'a str{
    if first.len()>second.len(){
        return first
    }
    second
}



pub(crate) fn life_time() {
    println!("{}", find_longest("evren", "okcu"));
    life_time_on_struct();
    life_time_with_self();
    write_static_const();
}

fn write_static_const()  {
    println!("{}",x);
}

fn life_time_with_self()  {
    let c=Coffee{
        name:"Flat White",
    };
    println!("{}",c.get_desc("evren"));
}

struct Coffee<'a>{
    name:&'a str,
}
impl<'a> Coffee<'a>{
    fn get_desc(&self,another: &str)->&str{
        self.name
    }
}

fn life_time_on_struct() {
    let p=Person{
        name:"evren",
    };
    println!("{:#?}",&p);
}
#[derive(Debug)]
struct Person<'a>{
    name:&'a str,
}
//mod static_lifetime{
    const x:&'static str="evren";
    static mut y:&'static str="okcu";
//}