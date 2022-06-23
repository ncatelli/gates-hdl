use scrap::prelude::v1::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

type RuntimeResult<T> = Result<T, RuntimeError>;

enum RuntimeError {
    FileNotSpecified,
    FileUnreadable,
    Undefined(String),
}

impl std::fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotSpecified => write!(f, "source file not specified"),
            Self::FileUnreadable => write!(f, "source file unreadable"),
            Self::Undefined(s) => write!(f, "{}", s),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

fn read_src_file<P: AsRef<Path>>(filename: P) -> RuntimeResult<String> {
    let mut f = File::open(filename).map_err(|_| RuntimeError::FileUnreadable)?;

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(RuntimeError::Undefined(e.to_string())),
    }
}

enum FileOrStdOut {
    File(File),
    StdOut,
}

impl Write for FileOrStdOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            FileOrStdOut::File(f) => f.write(buf),
            FileOrStdOut::StdOut => std::io::stdout().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            FileOrStdOut::File(f) => f.flush(),
            FileOrStdOut::StdOut => std::io::stdout().flush(),
        }
    }
}

fn write_dest_file(mut f: FileOrStdOut, data: &[u8]) -> RuntimeResult<()> {
    match f.write_all(data) {
        Ok(_) => Ok(()),
        Err(e) => Err(RuntimeError::Undefined(e.to_string())),
    }
}

fn main() -> RuntimeResult<()> {
    let raw_args: Vec<String> = env::args().into_iter().collect::<Vec<String>>();
    let args = raw_args.iter().map(|a| a.as_str()).collect::<Vec<&str>>();

    // Flag Definitions
    let help = scrap::Flag::store_true("help", "h", "display usage information.").optional();
    let out_file = scrap::Flag::expect_string("out-file", "o", "an output path.").optional();

    let cmd = scrap::Cmd::new("gates-hdl")
        .description("A \"HDL\" for the gates simulator.")
        .author("Nate Catelli <ncatelli@packetfire.org>")
        .version("0.1.0")
        .with_flag(out_file)
        .with_flag(help)
        .with_args_handler(|args, (ouf, _)| {
            let src_file = if args.len() == 1 { args.get(0) } else { None }
                .ok_or(RuntimeError::FileNotSpecified)?;

            let build_ctx = read_src_file(&(src_file.value))
                .and_then(|input| {
                    gates_hdl::parse(&input)
                        .map_err(|e| RuntimeError::Undefined(format!("{:?}", e)))
                })
                .and_then(|ast| {
                    gates_hdl::check(ast).map_err(|e| RuntimeError::Undefined(format!("{:?}", e)))
                })?;

            let output =
                gates_hdl::compiler::compile(build_ctx).map_err(RuntimeError::Undefined)?;

            let sink = ouf
                .map(|filename| {
                    let file_name = Path::new(&filename).to_path_buf();
                    OpenOptions::new()
                        .truncate(true)
                        .create(true)
                        .write(true)
                        .open(file_name)
                        .map_err(|_| RuntimeError::FileUnreadable)
                })
                .map_or(Ok(FileOrStdOut::StdOut), |f_res| {
                    f_res.map(FileOrStdOut::File)
                })?;

            write_dest_file(sink, output.as_bytes())
        });

    cmd.evaluate(&args[..])
        .map_err(|e| RuntimeError::Undefined(format!("{}\n{}", e, cmd.help())))
        .and_then(
            |scrap::Value {
                 span,
                 value: (flags, help),
             }| {
                if help.is_some() {
                    println!("{}", cmd.help());
                    Ok(())
                } else {
                    let unmatched_args = scrap::return_unused_args(&args[..], &span);
                    cmd.dispatch_with_args(unmatched_args, Value::new(span, (flags, help)))
                        .map(|_| ())
                }
            },
        )
}
