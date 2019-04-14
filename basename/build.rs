use std::env;

use clap::{ App, Shell, load_yaml };

fn main() {
    let yaml = load_yaml!("src/basename.yml");
    let mut app = App::from_yaml(yaml);

    let out_dir = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        _ => return
    };

    app.gen_completions("basename", Shell::Zsh, out_dir.clone());
    app.gen_completions("basename", Shell::Fish, out_dir.clone());
    app.gen_completions("basename", Shell::Bash, out_dir.clone());
    app.gen_completions("basename", Shell::PowerShell, out_dir.clone());
    app.gen_completions("basename", Shell::Elvish, out_dir)
}
