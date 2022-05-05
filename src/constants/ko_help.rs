extern crate colored;
use crate::constants::ko_help::colored::Colorize;

pub fn korean_help_message() -> String {
  return format!(
    "
{usage}: {tlqkf} [qjwjs] [ehdna] [..] [;;] [rkwmdk!]

{subcommand}:
  {version}
                        {tlqkf} 버전을 한국어로 출력합니다
  {help}
                        {tlqkf} 사용법을 한국어로 출력합니다
  {repeat}
                        {tlqkf} 취소하기 전 까지 반복적으로 실행됩니다
  {yes}
                        방금 실패한 커멘드를 물어보지 않고 곧장 {tlqkf}합니다
",
    usage = "사용법".green(),
    subcommand = "커맨드".green(),
    tlqkf = "시발".yellow(),
    version = "qjwjs (버전)".cyan(),
    help = "ehdna (도움)".cyan(),
    repeat = "., .., ...".cyan(),
    yes = ";, ;;, ;;;, rkwmdk, rkwmdk! (가즈아!)".cyan(),
  );
}
