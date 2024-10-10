
pub(crate) fn associated_types(){
    assiciated_types1();
    assiciated_types2();
    
}

fn assiciated_types1() {
    let s=AnyStruct{
        s: "Evren".to_string(),
        i: 34,
    };
    let result=  s.combine("Evren".to_string(), 2);
    println!("combimation result:{}",result);
}

trait Combiner {
    type T1;
    type T2;

    fn combine(&self,x:Self::T1,y:Self::T2)->String;
}

struct AnyStruct{
    s:String,
    i:i8,
}

impl Combiner for AnyStruct{
    type T1 =  String;

    type T2=i8;

    fn combine(&self, x:Self::T1,y:Self::T2)->String {
        format!("{x}:{y}")
    }
}


trait Incrementor {
    type T1;
    type T2;
    fn increment(&mut self, t1:Self::T1,t2:Self::T2)->String;
}

struct Counter{
    count:i32,
}

impl Incrementor for Counter {
    type T1=i8;

    type T2=i32;

    fn increment(&mut self,t1:Self::T1,t2:Self::T2)->String {
        self.count=t1 as i32+t2;
        let res=&self.count;
        res.to_string()
    }
}
fn assiciated_types2(){
    let mut c=Counter{
        count: 0,
    };
    println!("associated type increment 1:{}",c.increment(1,2));
    println!("associated type increment 1:{}",c.increment(3,4));
}