use std::fmt::Display;




struct Id(String);

pub(crate) fn newtype()  {
    let id=Id("evren".to_string());
    println!("{}",id);
}

impl Display for Id{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Writing Id: {}",self.0)
    }
}