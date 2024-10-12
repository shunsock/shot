mod loader;
mod receiver;

use loader::Loader;
use receiver::Receiver;

fn main() {
    let receiver = Receiver::new();
    let received_data = receiver.receive();

    let loader = match Loader::load(received_data.source_code, received_data.file_path) {
        Ok(source_code_loader) => source_code_loader,
        Err(error) => {
            eprintln!("{:?}", error.to_string());
            std::process::exit(1);
        }
    };

    println!("{}", loader.source_code);
    println!("{}", received_data.debug_mode);
}
