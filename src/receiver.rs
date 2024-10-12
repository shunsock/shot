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

const ASCII_ART: &str = r#"
     _______. __    __    ______   .___________.
    /       ||  |  |  |  /  __  \  |           |
   |   (----`|  |__|  | |  |  |  | `---|  |----`
    \   \    |   __   | |  |  |  |     |  |
.----)   |   |  |  |  | |  `--'  |     |  |
|_______/    |__|  |__|  \______/      |__|
                    ##
             #####%%##%%%#####
               %%#%@@%#%##%%%%#####%%########%%%%%
###+#######*-+*###****######***#%%%#%@%
%%%%%%#######** %%%             %%%
%%%###*##   #
 %
"#;

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
        Command::new("shot")
            .about("kill your task in one shot")
            .author("shunsock")
            .version("0.1.0")
            .help_template(&format!(
                "\n{}\n\n{{before-help}}{{about}}\n\nUSAGE:\n    {{usage}}\n\n{{all-args}}{{after-help}}\n",
                ASCII_ART
            ))
            .arg(
                Arg::new("inline")
                    .required(false)
                    .short('i')
                    .help("read source code inline"),
            )
            .arg(
                Arg::new("file")
                    .required(false)
                    .short('f')
                    .help("read source code from file"),
            )
            .arg(
                Arg::new("debug")
                    .short('d')
                    .long("debug")
                    .action(ArgAction::SetTrue)
                    .help("Enable debug mode"),
            )
    }
}
