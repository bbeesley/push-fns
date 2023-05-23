use std::{fs, io::Read, path::PathBuf};
use {
  clap::CommandFactory,
  clap_complete::{Generator, Shell},
  std::{env, ffi::OsStr},
};

include!("src/args.rs");

fn write_completions_file<G: Generator + Copy, P: AsRef<OsStr>>(generator: G, out_dir: P) {
  let mut args = Cli::command();
  {
    let name = &"push-fns";
    clap_complete::generate_to(generator, &mut args, name.to_string(), &out_dir)
      .expect("clap complete generation failed");
  }
}

fn write_man_file<P: AsRef<OsStr>>(out_dir: P) -> std::io::Result<()> {
  let args = Cli::command();
  let man = clap_mangen::Man::new(args);
  let mut buffer: Vec<u8> = Default::default();
  man.render(&mut buffer)?;
  std::fs::write(PathBuf::from(out_dir.as_ref()).join("push-fns.1"), buffer)
}

fn write_readme_file() -> std::io::Result<()> {
  let mut header_file = fs::File::open("README_HEADER.md").expect("Unable to open file");
  let mut header_contents = String::new();
  header_file
    .read_to_string(&mut header_contents)
    .expect("Unable to read file");
  let command_line_help = clap_markdown::help_markdown::<Cli>();
  let lines = command_line_help.lines();
  let mut help_content = String::new();
  for line in lines.skip(3) {
    help_content.push_str(&format!("{}\n", line));
  }
  let command_line_help = format!("{}\n{}", header_contents, help_content);
  std::fs::write(PathBuf::from("README.md"), command_line_help.as_bytes())
}

/// write the shell completion scripts and man pages which will be added to
/// the release archive
fn build() {
  let out_dir = env::var_os("OUT_DIR").expect("out dir not set");
  write_completions_file(Shell::Bash, &out_dir);
  write_completions_file(Shell::Elvish, &out_dir);
  write_completions_file(Shell::Fish, &out_dir);
  write_completions_file(Shell::PowerShell, &out_dir);
  write_completions_file(Shell::Zsh, &out_dir);
  let _ = write_readme_file();
  let _ = write_man_file(&out_dir);

  eprintln!("completion scripts and manpage generated in {:?}", out_dir);
}

fn main() {
  build();
}
