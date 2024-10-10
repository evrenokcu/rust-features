

pub trait Add2<RHS=Self>{
    type Output;
    fn add2(&self,other:&RHS)->Self::Output;
}
#[derive(Debug)]
struct Location{
    x:i32,
    y:i32,
}

impl Add2 for Location{
    type Output=Location;

    fn add2(&self,other:&Self)->Self::Output {
       Location{
        x: self.x+other.x,
        y: self.y+other.y,
        }
    }
}
impl Add2<i8> for Location{
    type Output=Location;

    fn add2(&self,other:&i8)->Self::Output {
        let t=*other as i32;
        Location{
            x:self.x+&(*other as i32),
            y:self.y+&(*other as i32),

        }
    }
}

pub(crate) fn default_generic_type()  {
    default_generic_type1();
    non_default_generic_type();
}

fn non_default_generic_type()  {
   let l1=Location{
    x:1,
    y:2,
   };
   let x:i8=1;
   let l1=l1.add2(&x);
   println!("{:?}",l1);
}

fn default_generic_type1() {
    let l1=Location{
        x:1,
        y:1,
    };
    let l2=Location{
        x:2,
        y:3,
    };
    let l3=l1.add2(&l2);
    println!{"added locations: {:?}", l3};
}

