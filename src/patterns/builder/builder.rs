pub trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}
pub trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}
#[derive(Debug)]
pub struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color: String,
}
impl Bicycle {
    fn make(&self) -> &String {
        &self.make
    }
    fn model(&self) -> &String {
        &self.model
    }
    fn size(&self) -> i32 {
        self.size
    }
    fn color(&self) -> &String {
        &self.make
    }
}
impl Buildable<Bicycle, BicycleBuilder> for Bicycle {
    fn builder() -> BicycleBuilder {
        BicycleBuilder::new()
    }
}

pub struct BicycleBuilder {
    bycle: Bicycle,
}
impl Builder<Bicycle> for BicycleBuilder {
    fn new() -> Self {
        Self {
            bycle: Bicycle {
                make: String::new(),
                model: String::new(),
                size: 0,
                color: String::new(),
            },
        }
    }
    fn build(self) -> Bicycle {
        self.bycle
    }
}
impl BicycleBuilder {
    pub fn with_make(self, make: &str) -> Self {
        Self {
            bycle: Bicycle {
                make: make.into(),
                ..self.bycle
            },
        }
    }
    pub fn with_model(self, model: &str) -> Self {
        Self {
            bycle: Bicycle {
                model: model.into(),
                ..self.bycle
            },
        }
    }
    pub fn with_size(self, size: i32) -> Self {
        Self {
            bycle: Bicycle {
                size: size,
                ..self.bycle
            },
        }
    }
    pub fn with_color(self, color: &str) -> Self {
        Self {
            bycle: Bicycle {
                color: color.into(),
                ..self.bycle
            },
        }
    }
    //not &self or &mut self
}
