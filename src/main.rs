use std::io::Write;
//бере значення у юзера
fn get_input(query: &str) -> String{
    print!("{query}");
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_owned()
}


fn main() {
    loop {
        println!("########################");

        let input_file_name = get_input("Enter file name to process: ");
        let input_file = match std::fs::File::open(&input_file_name) {
            Ok(file) => {}
            Err(err) => {
                println!("Can't open file {}", err);
                continue
            }
        };

        let key = match get_input("Enter a key for file encryption/decryption: ")
            .parse()::<u8>() {
            Ok(key) => key,
            Err(err) => {
                match err.kind {

                }
                continue
            }
        };

        println!();

    }
}
