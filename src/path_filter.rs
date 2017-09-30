
use std::path::{PathBuf, Path, Component};
use ignore::gitignore::GitignoreBuilder;
use errors;

pub(crate) fn new(base_path: &Path, stash_path: &Path) -> errors::Result<PathFilter> {
    let base_path = PathBuf::from(base_path).canonicalize()?;
    let stash_path = PathBuf::from(stash_path).canonicalize()?;
    Ok(PathFilter {
        base_path: base_path,
        stash_path: stash_path
    })
}

pub(crate) struct PathFilter {
    base_path: PathBuf,
    stash_path: PathBuf,
}

impl PathFilter {
    fn is_in_scope(&self, path: &Path) -> bool {
        path.is_absolute() && path.starts_with(&self.base_path)
    }

    fn ignored(&self, path: &Path) -> errors::Result<bool> {
        let rel_path = path.strip_prefix(&self.base_path)?;

        let mut builder = GitignoreBuilder::new(&self.base_path);
        builder.add(".gitignore");
        
        let mut ignore_path = self.base_path.clone();
        for c in rel_path.components() {
            match c {
                Component::Normal(ref path_part) => ignore_path.push(path_part),
                _ => panic!("Already checked this was a canonical path relative to the filter base; should only contain normal components")
            }
            if ignore_path.is_file() {
                break;
            }

            builder.add(&ignore_path.join(".gitignore"));
        }

        let built = builder.build()?;

        Ok(built.matched_path_or_any_parents(&path, path.is_dir()).is_ignore())
    }

    fn is_stash_path(&self, path: &Path) -> bool {
        path.starts_with(&self.stash_path)
    }
 
    pub fn allow(&self, path: &Path) -> errors::Result<bool> {
        let path = path.canonicalize();
        if let Ok(path) = path {
            if !self.is_in_scope(&path) {
                return Ok(false);
            }

            if self.is_stash_path(&path) {
                return Ok(false);
            }

            if self.ignored(&path)? {
                return Ok(false);
            }


            return Ok(true);
        }
        Ok(false)

    }
}

