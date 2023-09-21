use nix::{
    fcntl::{self, FcntlArg, OFlag},
    libc, unistd,
};
use statusline::{default, Environment, IconMode, Style};
use std::{env, fs, io, io::Write};
use unicode_width::UnicodeWidthStr;

fn readline_width(s: &str) -> usize {
    let mut res = s.width();
    for (i, c) in s.bytes().enumerate() {
        match c {
            b'\x01' => res += i + 1,
            b'\x02' => res -= i,
            _ => {}
        }
    }
    res
}

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
                libc::fcntl(3, libc::F_SETOWN, unistd::getpid());
            }
            fcntl::fcntl(3, FcntlArg::F_SETFL(OFlag::O_ASYNC)).unwrap();

            use statusline::BlockType; //<===

            let mode = IconMode::build();
            let args = Environment::from_env(&args.collect::<Vec<String>>());
            let bottom = default::bottom(&args);

            let mut line = default::top(&args);

            let line_length: usize = line
                .iter()
                .map(|x| x.pretty(&mode))
                .filter_map(|x| x)
                .map(|x| readline_width(&x))
                .sum();

            if line_length + 25 >= term_size::dimensions().map(|s| s.0).unwrap_or(80) {
                // three lines
                let mut second = BlockType::Empty.create_from_env(&args);
                std::mem::swap(&mut second, &mut line[5]);
                let second = [BlockType::Continue.create_from_env(&args), second];

                eprint!(
                    "\n\n\n{}",
                    default::pretty(&line, &mode)
                        .join_lf(default::pretty(&second, &mode))
                        .clear_till_end()
                        .prev_line(2)
                        .save_restore()
                );

                print!(
                    "{}{}",
                    default::title(&args).invisible(),
                    default::pretty(&bottom, &mode)
                );
                io::stdout().flush().unwrap();
                unistd::close(1).unwrap();

                let line = default::extend(line);
                let second = default::extend(second);
                eprint!(
                    "{}",
                    default::pretty(&line, &mode)
                        .join_lf(default::pretty(&second, &mode))
                        .clear_till_end()
                        .prev_line(2)
                        .save_restore()
                );
            } else {
                // two lines
                eprint!(
                    "\n\n{}",
                    default::pretty(&line, &mode)
                        .clear_till_end()
                        .prev_line(1)
                        .save_restore()
                );

                print!(
                    "{}{}",
                    default::title(&args).invisible(),
                    default::pretty(&bottom, &mode)
                );
                io::stdout().flush().unwrap();
                unistd::close(1).unwrap();

                let line = default::extend(line);
                eprint!(
                    "{}",
                    default::pretty(&line, &mode)
                        .clear_till_end()
                        .prev_line(1)
                        .save_restore()
                );
            }
        }
        _ => {
            let ver = env!("CARGO_PKG_VERSION");
            let apply_me = format!("eval \"$(\"{exec}\" --env)\"");
            println!("[statusline {ver}] --- bash status line, written in Rust");
            println!(">> https://git.yukii.keenetic.pro/yuki0iq/statusline");
            println!("Simple install:");
            println!("    echo '{apply_me}' >> ~/.bashrc");
            println!("    source ~/.bashrc");
            println!("Test now:");
            println!("    {apply_me}");
        }
    }
}
