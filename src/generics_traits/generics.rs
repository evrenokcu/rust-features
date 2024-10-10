use std::{fmt::{write, Display}, ops::Add};


fn add<T:Add<Output = T>>(v1:T, v2:T)->T{
    v1+v2
}
 pub(super) fn add_numbers()  {
    let v1=1;
    let v2=2;
   println!("{}+{}={}",&v1,&v2,add(v1, v2));
}


pub (super) fn return_trait()->impl Drink {
    Coffee{}
}
pub (super) fn return_multiple_trait(drink_type:i8)->Box< dyn Drink> {
    match drink_type {
        1 =>Box::new(Coffee{}),
        _ =>Box::new( Tea{})
        
    }
}

pub(crate) fn traits_generics() {
    
    let c=Coffee{};
    let t=Tea{};

    println!("{}",brew_dring_generics(&c));
    println!("{}",brew_dring_trait(&t));
    print_drink_trait(&c);
    print_drink_generics1(&t);
    print_drink_generics2(&c);
    conditional_implementations();

}

fn conditional_implementations()  {
    let coordinateI32:Coordinate<i32>=Coordinate{
        x: 3,
        y: 34,
    };

    let coordinatei8:Coordinate<i8>=Coordinate{
        x:5,
        y:34,
    };

    //coordinateI32.printi8// only implemented for i8
    coordinatei8.printi8();
    coordinatei8.printDisplay();
    coordinateI32.printDisplay2();
}

struct Coordinate<T>{
    x:T,
    y:T,
}
impl Coordinate<i8>{
    fn printi8(&self){
        println!("Coordinate i8:{}:{}",self.x, self.y);
    }
}

impl<T:Display> Coordinate<T>{
       fn printDisplay(&self){
            println!("Coordinate Display:{}",self.x);
       }
}

impl<T> Coordinate<T>
where T:Display{
    fn printDisplay2(&self){
        println!("Coordinate Display2:{}",self.x);
   }
}

pub(super) trait Drink {
    fn brew(&self)->String;
}

pub (super)struct Coffee{

}
impl Drink for Coffee{
    fn brew(&self)->String {
        String::from("brewing coffee")
    }
}

pub (super) struct Tea{

}
impl Drink for Tea{
    fn brew(&self)->String {
        String::from("brewing tea")
    }
}
fn brew_dring_trait(drink:&impl Drink)->String{
    drink.brew()
}

fn brew_dring_generics<T:Drink>(drink:&T)->String{
    drink.brew()
}

fn print_drink_trait(drink:&(impl Drink+Display)){
    println!("{}",drink.brew());
}
fn print_drink_generics1<T:Drink+Display>(drink:&T){
    println!("{}",drink.brew());
}
fn print_drink_generics2<T>(drink:&T)
    where T:Drink+Display
{
    println!("{}",drink.brew());
}
impl Display for Tea{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Tea")
    }
}

impl Display for Coffee{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Coffee")
    }
}
