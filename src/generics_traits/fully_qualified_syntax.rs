

struct Printer{
}

impl Printer{
    fn write(&self){
        println!("Printer.Write");
    }
}

trait Logger {
    fn write(&self){
        println!("Logger.Write");
    }
}

trait File{
    fn write(&self){
        println!("File.Write");
    }
}
impl File for Printer{}
impl Logger for Printer{}

pub(crate) fn fully_qualified_syntax()  {
    with_self();
    without_self();
}
fn with_self() {
    let p=Printer{};
    p.write();
    Printer::write(&p);
    File::write(&p);
    Logger::write(&p);
}
fn without_self() {
    let p=Printer2{};

    Printer2::write();
    <Printer2 as File2>::write();
    <Printer2 as Logger2>::write();
}


struct Printer2{
}

impl Printer2{
    fn write(){
        println!("Printer.Write");
    }
}

trait Logger2 {
    fn write(){
        println!("Logger.Write");
    }
}

trait File2{
    fn write(){
        println!("File.Write");
    }
}
impl File2 for Printer2{}
impl Logger2 for Printer2{}
    