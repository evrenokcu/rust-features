pub(crate) fn execute() {
    let b = BitCount(2);
    println!("{:?}", &b);
    println!("{:?}", b.to_bytes())
}
#[derive(Debug)]
struct BitCount(i32);
#[derive(Debug)]
struct ByteCount(i32);

impl BitCount {
    fn to_bytes(&self) -> ByteCount {
        ByteCount(&self.0 / 8)
    }
}

impl ByteCount {
    fn to_bits(&self) -> BitCount {
        BitCount(&self.0 * 8)
    }
}
