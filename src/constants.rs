use colored::*;

pub fn print_korean_help_message() {
    println!(
        "
{USAGE}
  tlqkf <SUBCOMMAND>

{OPTIONS}
  {help}       도움말을 출력합니다
  {version}    버전을 출력합니다

{SUBCOMMANDS}
  {ehdna}      도움말을 출력합니다
",
        USAGE = "USAGE:".yellow(),
        OPTIONS = "OPTIONS:".yellow(),
        SUBCOMMANDS = "SUBCOMMANDS:".yellow(),
        help = "-h, --help".green(),
        version = "-V, --version".green(),
        ehdna = "ehdna, help".green(),
    );
}
