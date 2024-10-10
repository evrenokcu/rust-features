use std::{ sync::mpsc, thread::{self, Thread}};



pub(crate) fn send_sync()  {
    
    let (ts,tr)=mpsc::channel();
    thread::spawn(move ||{
        let c=Coffee{
            name: String::from("Black Coffee"),
            quantity: 1,
        };
        ts.send(c).unwrap()
    });

    thread::spawn(move ||{
    let c=tr.recv().unwrap();
    println!("{:#?}",c);
    });
    
    ;

}

#[derive(Debug)]
struct Coffee{
    name:String,
    quantity:i8,
}
impl Clone for Coffee{
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), quantity: self.quantity }
    }
}


#[derive(Debug, Clone,Copy)]
struct Coffee2{
    name:i32,
    quantity:i8,
}