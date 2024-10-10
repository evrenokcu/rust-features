use std::{
    fs::{read, File},
    io::{BufRead, BufReader, Error, Seek, SeekFrom, Write},
};

use crate::print_header;

trait Command {
    fn execute(&self) -> Result<(), Error>;
}

struct ReadFileCommand {
    receiver: File,
}
impl ReadFileCommand {
    fn new(receiver: File) -> Self {
        Self { receiver }
    }
}
impl Command for ReadFileCommand {
    fn execute(&self) -> Result<(), Error> {
        let mut reader = BufReader::new(&self.receiver);
        reader.seek(SeekFrom::Start(0))?;
        for (count, line) in reader.lines().enumerate() {
            println!("{:2}: {}", count + 1, line?)
        }
        Ok(())
    }
}

struct WriteFileCommand {
    receiver: File,
    content: String,
}
impl WriteFileCommand {
    fn new(content: String, file: File) -> Self {
        Self {
            content,
            receiver: file,
        }
    }
}

impl Command for WriteFileCommand {
    fn execute(&self) -> Result<(), Error> {
        let mut writer = self.receiver.try_clone()?;
        writer.write_all(self.content.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
}

pub(crate) fn execute() {
    print_header("Command Pattern");
    if let Err(error) = execute_list() {
        println!("{:#?}", error);
    }
}

fn execute_list() -> Result<(), Error> {
    let f = File::options()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("write_to_file.txt")?;

    let write_command =
        WriteFileCommand::new("this is a sample content".to_string(), f.try_clone()?);
    let read_command = ReadFileCommand::new(f.try_clone()?);
    let commands: Vec<Box<dyn Command>> = vec![Box::new(write_command), Box::new(read_command)];

    for command in commands {
        command.execute()?;
    }

    Ok(())
}
