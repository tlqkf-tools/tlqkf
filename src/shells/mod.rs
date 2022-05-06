use sysinfo::{ProcessExt, System, SystemExt};

pub fn shell() -> String {
    let env_shell = get_shell_from_env();

    if env_shell == "" {
        return get_shell_from_proc();
    }

    return env_shell;
}

fn get_shell_from_env() -> String {
    let shell = std::env::var("TLQKF_SHELL");
    if shell.is_ok() {
        return String::from(shell.unwrap());
    }
    return String::from("");
}

fn get_shell_from_proc() -> String {
    let system = System::new_all();
    let shells = ["bash", "fish", "zsh", "csh", "tcsh", "powershell", "pwsh"];

    let process = system
        .processes()
        .values()
        .into_iter()
        .find(|process| shells.contains(&process.name()));

    if process.is_some() {
        return String::from(process.unwrap().name());
    }

    return String::from("Generic Shell");
}
