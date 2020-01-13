use std::env;

use clap::{load_yaml, App, Shell};

fn main() {
    let yaml = load_yaml!("src/template.yml");
    let mut app = App::from_yaml(yaml);

    let out_dir = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        _ => return,
    };

    app.gen_completions("template", Shell::Zsh, out_dir.clone());
    app.gen_completions("template", Shell::Fish, out_dir.clone());
    app.gen_completions("template", Shell::Bash, out_dir.clone());
    app.gen_completions("template", Shell::PowerShell, out_dir.clone());
    app.gen_completions("template", Shell::Elvish, out_dir);
}
