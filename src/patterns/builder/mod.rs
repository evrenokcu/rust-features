use builder::{Buildable, Builder};

mod builder;
pub(crate) fn execute() {
    crate::print_header("builder");
    build_bicycle();
}

fn build_bicycle() {
    let b = builder::Bicycle::builder()
        .with_make("cannondale")
        .with_color("yellow")
        .with_model("f32")
        .with_size(54)
        .build();
    println!("Bicycle: {:?}", b);
}
