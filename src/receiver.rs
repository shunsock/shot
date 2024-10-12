use clap::{Arg, ArgAction, Command};

pub struct Receiver {}

pub struct ReceivedData {
    pub source_code: Option<String>,
    pub file_path: Option<String>,
    pub debug_mode: bool,
}

impl Default for Receiver {
    fn default() -> Self {
        Self::new()
    }
}

impl Receiver {
    pub fn new() -> Self {
        Receiver {}
    }

    pub fn receive(&self) -> ReceivedData {
        let command: Command = Receiver::load_command_settings();
        let matches = command.get_matches();

        // コマンドライン引数を取り出す
        let source_code: Option<String> = matches.get_one::<String>("expression").cloned();
        let file_path: Option<String> = matches.get_one::<String>("file").cloned();
        let debug_mode: bool = matches.get_flag("debug");

        ReceivedData {
            source_code,
            file_path,
            debug_mode,
        }
    }

    pub fn load_command_settings() -> Command {
        Command::new("rast")
            .about("kill your task in one shot")
            .author("shunsock")
            .version("0.1.0")
            .arg(Arg::new("expression").required(false).short('e'))
            .arg(Arg::new("file").required(false).short('f'))
            .arg(
                Arg::new("debug")
                    .short('d')
                    .long("debug")
                    .action(ArgAction::SetTrue)
                    .help("Enable debug mode"),
            )
    }
}
