extern crate colored;
use crate::constants::en_help::colored::Colorize;

pub fn english_help_message() -> String {
  return format!(
    "
{usage}: {tlqkf} [-v] [-h] [-r] [-y]
             [--version] [--help] [--repeat] [--yes]
             [qjwjs] [ehdna] [..] [;;] [rkwmdk!]

{options}:
  {version}
                        show program's version and exit
  {help}
                        show this help message and exit
  {repeat}
                        repeat on failure
  {yes}
                        execute fixed command without confirmation

{subcommands}:
  {qjwjs}
                        show program's version(한국어) and exit
  {help_subcommand}
                        show this help message and exit
  {ehdna}
                        show this help message(한국어) and exit
  {repeat_subcommand}
                        repeat on failure
  {rkwmdk}
                        execute fixed command without confirmation
",
    usage = "usage".green(),
    options = "options".green(),
    subcommands = "sub commands".green(),
    tlqkf = "tlqkf".yellow(),
    version = "-v, --version".cyan(),
    help = "-h, --help".cyan(),
    repeat = "-r, --repeat".cyan(),
    yes = "-y, --yes".cyan(),
    qjwjs = "qjwjs".cyan(),
    help_subcommand = "help".cyan(),
    ehdna = "ehdna".cyan(),
    repeat_subcommand = "., .., ...".cyan(),
    rkwmdk = ";, ;;, ;;;, rkwmdk, rkwmdk!".cyan(),
  );
}
