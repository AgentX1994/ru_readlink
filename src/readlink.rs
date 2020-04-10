use crate::settings::CanonicalizeOption;
use std::ffi::OsString;
use std::io;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;
use std::path::{Component, Path, PathBuf};

pub fn read_link<P: AsRef<Path>>(
    path: P,
    canonicalize: &CanonicalizeOption,
) -> io::Result<PathBuf> {
    if canonicalize.is_none() {
        return path.as_ref().read_link();
    }

    let mut result_path = PathBuf::new();

    let mut first_component = true;
    let mut component_iter = path.as_ref().components().peekable();
    while let Some(component) = component_iter.next() {
        // skip the last component to allow special handling for canonicalization
        let last = component_iter.peek().is_none();

        match component {
            Component::Prefix(_) | Component::RootDir => result_path.push(component),
            Component::CurDir => {
                if first_component {
                    result_path = std::env::current_dir()?;
                }
            }
            Component::ParentDir => {
                if result_path.parent().is_none() {
                    result_path = std::env::current_dir()?;
                }
                result_path.pop();
            }
            Component::Normal(file) => {
                if first_component {
                    result_path = std::env::current_dir()?;
                }
                let mut this_path = result_path.clone();
                this_path.push(file);
                let res = this_path.read_link();
                match res {
                    Ok(p) => result_path.push(p),
                    Err(ref e) => match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            if canonicalize.is_missing() || (canonicalize.is_all_but_last() && last)
                            {
                                result_path.push(file);
                            } else {
                                return res;
                            }
                        }
                        std::io::ErrorKind::Other => {
                            // Not a directory error
                            if canonicalize.is_missing() && e.raw_os_error() == Some(20) {
                                result_path.push(file);
                            } else {
                                return res;
                            }
                        }
                        std::io::ErrorKind::InvalidInput => {
                            // This just means the file was not a link
                            result_path.push(file);
                        }
                        _ => return res,
                    },
                }
            }
        }
        first_component = false;
    }

    // if last byte is '/' remove it
    // There has to be a better way of doing this
    #[cfg(unix)]
    let os_string = {
        let mut path_vec = result_path.into_os_string().into_vec();
        if let Some(&b) = path_vec.last() {
            if b == b'/' {
                path_vec.pop();
            }
        }
        OsString::from_vec(path_vec)
    };

    // TODO remove any trailing '\' on windows
    #[cfg(windows)]
    let os_string = result_path.into_os_string();

    Ok(PathBuf::from(os_string))
}
