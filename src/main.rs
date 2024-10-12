mod loader;
mod receiver;
mod virtual_machine;

use crate::receiver::ReceivedData;
use crate::virtual_machine::VirtualMachine;
use loader::Loader;
use receiver::Receiver;

fn main() {
    let receiver: Receiver = Receiver::new();
    let received_data: ReceivedData = receiver.receive();

    let loader: Loader = match Loader::load(received_data.source_code, received_data.file_path) {
        Ok(source_code_loader) => source_code_loader,
        Err(error) => {
            eprintln!("{:?}", error.to_string());
            std::process::exit(1);
        }
    };

    let virtual_machine: VirtualMachine = VirtualMachine::new(
        loader.source_code,
        loader.source_code_vector,
        received_data.debug_mode,
    );
    virtual_machine.run();
}
