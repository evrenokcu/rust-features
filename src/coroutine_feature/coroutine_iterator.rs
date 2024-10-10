use std::io::{BufRead, Read};
use std::ops::{Coroutine, CoroutineState};
use std::{fs::File, io::BufReader, pin::Pin};

struct CargoReader {
    coroutine: Pin<Box<dyn Coroutine<Yield = (usize, String), Return = ()>>>,
}
impl CargoReader {
    fn new() -> std::io::Result<Self> {
        let file = File::open("Cargo.toml")?;
        let mut reader = BufReader::new(file);
        let mut line: usize = 0;

        let coroutine = Box::pin(
            #[coroutine]
            move || loop {
                let mut lineText = String::new();
                line += 1;
                match reader.read_line(&mut lineText) {
                    Ok(0) => return,
                    Ok(_) => yield (line, lineText),
                    _ => return,
                }
            },
        );
        // let coroutine = Box::pin(
        //     #[coroutine]
        //     move || loop {
        //         let mut line_text = String::new();
        //         line += 1;
        //         match reader.read_line(&mut line_text) {
        //             Ok(0) => return,
        //             Ok(_) => yield (line, line_text),
        //             _ => return,
        //         }
        //     },
        // );

        Ok(Self { coroutine })
    }
}

impl Iterator for CargoReader {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        match self.coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(val) => Some(val),
            CoroutineState::Complete(()) => None,
        }
    }
}

pub(crate) fn execute() {
    if let Err(e) = call_reader() {
        println!("error occured:{}", e);
    }
}

fn call_reader() -> std::io::Result<()> {
    let cargo_reader = CargoReader::new()?;

    for (line, text) in cargo_reader {
        print!("{line}: {text}");
    }

    Ok(())
}
