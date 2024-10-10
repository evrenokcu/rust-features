pub(crate) fn cow() {
    crate::print_header("cow");
    sample_1::sample();
    sample_2::sample();
    sample_3::sample();
}
mod sample_3 {
    use std::borrow::Cow;
    use std::collections::HashMap;

    use crate::print_header;

    // A struct to represent a configuration
    #[derive(Debug)]
    struct Config<'a> {
        settings: HashMap<&'a str, Cow<'a, str>>,
    }

    // Function to load and process the configuration
    fn process_config<'a>(config_data: HashMap<&'a str, &'a str>) -> Config<'a> {
        let mut settings = HashMap::new();

        for (key, value) in config_data {
            // For some settings, we need to modify them before storing
            if key == "log_directory" {
                // Normalize the log directory by appending a trailing slash
                let mut normalized = value.to_owned();
                if !normalized.ends_with('/') {
                    normalized.push('/');
                }
                settings.insert(key, Cow::Owned(normalized)); // Store modified (owned) data
            } else {
                // Store other settings as borrowed data to avoid unnecessary copying
                settings.insert(key, Cow::Borrowed(value));
            }
        }

        Config { settings }
    }

    pub(super) fn sample() {
        print_header("Configuration Sample, Cow");
        // Example config data from a file or user input (simulated here)
        let config_data = HashMap::from([
            ("username", "user123"),
            ("log_directory", "/var/logs"),
            ("max_connections", "100"),
        ]);

        // Process the configuration
        let config = process_config(config_data);

        // Output the final configuration
        for (key, value) in &config.settings {
            println!("{}: {}", key, value);
        }
    }
}
mod sample_2 {
    use std::borrow::Cow;

    use crate::print_header;

    pub(super) fn sample() {
        print_header("Cow sample 2");

        let borrowed = "borrowed";
        print_data(Cow::Borrowed(&borrowed));

        let owned = String::from("owned");
        print_data(Cow::Owned(owned));
    }

    fn print_data(input: Cow<str>) {
        println!("Cow Data:{}", input)
    }
}
mod sample_1 {

    use std::borrow::{Borrow, Cow};
    pub(super) fn sample() {
        crate::print_header("Cow sample 1");
        let x = remove_blanks("with blank");
        println!("{x}");

        let t = remove_blanks("noblank");
        println!("{t}");
        let y = Cow::Borrowed(&x);

        println!("{:?}", t)
    }

    fn remove_blanks(s: &str) -> Cow<str> {
        if s.contains(" ") {
            Cow::Owned(s.replace(" ", ""))
        } else {
            Cow::Borrowed(s)
        }
    }
}
