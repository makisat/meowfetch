use text_to_ascii_art::convert;

fn main() {
    let content = std::fs::read_to_string("/etc/os-release").expect("Failed to read file");

    let lines: Vec<&str> = content.split("\n").collect();

    for line in lines {
        if line.starts_with("NAME=") {
            let name = line.split("=").collect::<Vec<&str>>()[1];
            let name = name.replace("\"", "");

            let ascii_art = convert(name.to_owned()).unwrap();
            println!("{}", ascii_art);
        }
    }
}
