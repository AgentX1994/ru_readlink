use clap::{App, AppSettings, Arg};
/// readlink [OPTION]... FILE...
///
///        -f, --canonicalize
///               canonicalize by following every symlink in every component of the given name recursively; all but the last component must exist
///        
///        -e, --canonicalize-existing
///               canonicalize by following every symlink in every component of the given name recursively, all components must exist
///        
///        -m, --canonicalize-missing
///               canonicalize by following every symlink in every component of the given name recursively, without requirements on components existence
///        
///        -n, --no-newline
///               do not output the trailing delimiter
///        
///        -q, --quiet
///        
///        -s, --silent
///               suppress most error messages (on by default)
///        
///        -v, --verbose
///               report error messages
///        
///        -z, --zero
///               end each output line with NUL, not newline
///        
///        --help display this help and exit
///        
///        --version
///               output version information and exit
use std::path::PathBuf;

#[derive(Debug)]
pub enum CanonicalizeOption {
    AllButLast,
    Existing,
    Missing,
    None,
}

impl CanonicalizeOption {
    #[must_use]
    pub fn is_all_but_last(&self) -> bool {
        if let Self::AllButLast = self {
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_existing(&self) -> bool {
        if let Self::Existing = self {
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_missing(&self) -> bool {
        if let Self::Missing = self {
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_none(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    pub canonicalize: CanonicalizeOption,
    pub output_delimiter: bool,
    pub quiet: bool,
    pub zero: bool,
    pub files: Vec<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            canonicalize: CanonicalizeOption::None,
            output_delimiter: true,
            quiet: true,
            zero: false,
            files: Vec::<PathBuf>::new(),
        }
    }
}

impl Settings {
    #[must_use]
    pub fn from_args() -> Self {
        let matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("Rust reimplementation of readlink")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::with_name("canonicalize")
                .short("f")
                .long("canonicalize")
                .help("canonicalize by following every symlink in every component of the given name recursively; all but the last component must exist"))
            .arg(Arg::with_name("canonicalize existing")
                .short("e")
                .long("canonicalize-existing")
                .help("canonicalize by following every symlink in every component of the given name recursively; all components must exist")
                .overrides_with("canonicalize"))
            .arg(Arg::with_name("canonicalize missing")
                .short("m")
                .long("canonicalize-missing")
                .help("canonicalize by following every symlink in every component of the given name recursively; without requirements on components' existence")
                .overrides_with_all(&["canonicalize", "canonicalize existing"]))
            .arg(Arg::with_name("no delimiter")
                .short("n")
                .long("no-delimiter")
                .help("do not output the trailing delimiter"))
            .arg(Arg::with_name("zero")
                .short("z")
                .long("zero")
                .help("end each output line with NUL (\0), not newline"))
            .arg(Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .alias("silent")
                .help("suppress most error messages (on by default)"))
            .arg(Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("report error messages")
                .overrides_with("quiet"))
            .arg(Arg::with_name("file")
                .multiple(true)
                .required(true))
            .get_matches();

        let mut ret = Self::default();

        if matches.is_present("verbose") {
            ret.quiet = false;
        }

        if matches.is_present("no delimiter") {
            ret.output_delimiter = false;
        }

        if matches.is_present("zero") {
            ret.zero = true;
        }

        if matches.is_present("canonicalize") {
            ret.canonicalize = CanonicalizeOption::AllButLast;
        }

        if matches.is_present("canonicalize existing") {
            ret.canonicalize = CanonicalizeOption::Existing;
        }

        if matches.is_present("canonicalize missing") {
            ret.canonicalize = CanonicalizeOption::Missing;
        }

        ret.files = matches
            .values_of_os("file")
            .expect("No files!")
            .map(PathBuf::from)
            .collect();

        ret
    }
}
