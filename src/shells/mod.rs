use sysinfo::{ProcessExt, System, SystemExt};

pub fn shell() -> String {
    return get_shell_from_proc();
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
