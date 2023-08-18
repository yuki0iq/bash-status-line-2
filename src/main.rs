use libc::fcntl as fcntl_unsafe;
use nix::{
    fcntl::{self, FcntlArg, OFlag},
    unistd,
};
use statusline::{CommandLineArgs, StatusLine, Style};
use std::{env, fs, io, io::Write};

fn main() {
    let exec = fs::read_link("/proc/self/exe")
        .map(|pb| String::from(pb.to_string_lossy()))
        .unwrap_or("<executable>".to_owned());
    let mut args = env::args();
    args.next();
    match args.next().as_deref() {
        Some("--colorize") => match args.next() {
            Some(text) => println!("{}", text.colorize_with(&text).bold()),
            None => println!("`statusline --colorize <text>` to colorize string"),
        },
        Some("--env") => {
            println!("{}", include_str!("shell.sh").replace("<exec>", &exec));
        }
        Some("--run") => {
            unsafe {
                fcntl_unsafe(0, libc::F_SETOWN, unistd::getpid());
            }
            fcntl::fcntl(0, FcntlArg::F_SETFL(OFlag::O_ASYNC)).unwrap();

            let args = args.collect::<Vec<String>>();
            let line = StatusLine::from_env(CommandLineArgs::from_env(&args));

            let top_line =
                |line: &StatusLine| line.to_top().prev_line(1).save_restore().to_string();

            eprint!("{}", top_line(&line));

            print!("{}{}", line.to_title(None).invisible(), line.to_bottom());
            io::stdout().flush().unwrap();
            unistd::close(1).unwrap();

            let line = line.extended();
            eprint!("{}", top_line(&line));
        }
        _ => {
            println!("Bash status line --- written in rust. Add `eval \"$(\"{exec}\" --env)\"` to your .bashrc!");
        }
    }
}
