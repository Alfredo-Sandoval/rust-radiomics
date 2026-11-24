//! Private output staging shared by the public exporters.
//!
//! Mirrors `mlxradiomics/workflows/publication.py` (280 lines at mlx-radiomics b58783c).
//! Status: ported for the public exporters, minus the SIGTERM checkpoints.
//!
//! CPU orchestration on the calling thread: filesystem calls only, no numerical work.
//! All staged files stay on the destination filesystem, so publication is a rename or a
//! no-clobber hard link. Callers own the output-directory lock and decide which targets
//! may be replaced; [`Publication`] only stages files and restores replaced targets if a
//! later publication step fails.
//!
//! Differences from mlx, all in process-level behavior rather than in file contents:
//!
//! - The output lock is `File::try_lock` on a read-only handle of the directory (an
//!   `flock`), so it needs no lock file and is released when the handle closes.
//! - mlx defers the default main-thread SIGTERM through publication and rollback. This
//!   crate owns no signal handlers, so a terminating signal behaves as the host process
//!   defines; `Drop` still removes staging on unwinding, and [`reclaim_stale_staging`]
//!   clears what a killed process leaves behind.
//! - The runner (`mlxradiomics.medmlx.run`) is not ported; only the public exporters
//!   use this module.

use crate::imaging::io::has_detached_data;
use crate::{Error, Result};
use std::collections::HashSet;
use std::fs::{self, File};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const MARKER: &str = ".mlxradiomics-staging";
const STAGING_PREFIX: &str = ".mlxradiomics-";

fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::InvalidInput(message.into()))
}

/// Absolute path with `..` and `.` removed and every existing prefix symlink-resolved
/// (`Path.expanduser().resolve()` with `strict=False`).
pub fn resolve(path: &Path) -> PathBuf {
    let expanded = match path.to_str().and_then(|s| s.strip_prefix("~")) {
        Some(rest) if rest.is_empty() || rest.starts_with('/') => std::env::var_os("HOME")
            .map(|home| PathBuf::from(home).join(rest.trim_start_matches('/')))
            .unwrap_or_else(|| path.to_path_buf()),
        _ => path.to_path_buf(),
    };
    let absolute = std::path::absolute(&expanded).unwrap_or(expanded);
    let mut normal = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::ParentDir => {
                normal.pop();
            }
            Component::CurDir => {}
            other => normal.push(other.as_os_str()),
        }
    }
    // Resolve the longest existing prefix, then append the not-yet-created rest.
    let mut rest = Vec::new();
    let mut head = normal.clone();
    loop {
        if let Ok(real) = head.canonicalize() {
            let mut result = real;
            result.extend(rest.iter().rev());
            return result;
        }
        match head.file_name().map(|n| n.to_os_string()) {
            Some(name) => {
                rest.push(name);
                if !head.pop() {
                    return normal;
                }
            }
            None => return normal,
        }
    }
}

/// An exclusive advisory lock on an output directory, released on drop.
#[derive(Debug)]
pub struct OutputLock {
    _handle: File,
}

/// Create `directory` if needed and refuse overlapping runs using it
/// (`output_lock`).
///
/// # Errors
/// [`Error::InvalidInput`] if the directory cannot be created or opened, or another run
/// holds the lock.
pub fn output_lock(directory: &Path) -> Result<OutputLock> {
    let handle = fs::create_dir_all(directory)
        .and_then(|()| File::open(directory))
        .map_err(|_| {
            Error::InvalidInput(format!("invalid output directory: {}", directory.display()))
        })?;
    match handle.try_lock() {
        Ok(()) => Ok(OutputLock { _handle: handle }),
        Err(std::fs::TryLockError::WouldBlock) => invalid(format!(
            "another run is using output directory: {}",
            directory.display()
        )),
        Err(std::fs::TryLockError::Error(_)) => {
            invalid(format!("invalid output directory: {}", directory.display()))
        }
    }
}

#[cfg(unix)]
fn identity(path: &Path) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    path.metadata().ok().map(|m| (m.dev(), m.ino()))
}

#[cfg(not(unix))]
fn identity(_path: &Path) -> Option<(u64, u64)> {
    None
}

/// Refuse output paths resolving to, or hard-linked to, an input file
/// (`reject_input_aliases`).
pub fn reject_input_aliases(outputs: &[PathBuf], inputs: &[PathBuf]) -> Result<()> {
    for output in outputs {
        for source in inputs {
            let same_file = output.exists()
                && matches!((identity(output), identity(source)), (Some(a), Some(b)) if a == b);
            if resolve(output) == resolve(source) || same_file {
                return invalid(format!("output aliases an input: {}", output.display()));
            }
        }
    }
    Ok(())
}

/// Refuse outputs sharing a resolved path or an existing file identity
/// (`reject_output_aliases`).
pub fn reject_output_aliases(outputs: &[PathBuf]) -> Result<()> {
    let mut resolved = HashSet::new();
    let mut identities = HashSet::new();
    for output in outputs {
        if !resolved.insert(resolve(output)) {
            return invalid(format!(
                "output aliases another output: {}",
                output.display()
            ));
        }
        if output.exists()
            && let Some(id) = identity(output)
            && !identities.insert(id)
        {
            return invalid(format!(
                "output aliases another output: {}",
                output.display()
            ));
        }
    }
    Ok(())
}

/// Reject non-file targets and linked map directories, and create the parents, before
/// extraction (`prepare_output_targets`).
pub fn prepare_output_targets(directory: &Path, outputs: &[PathBuf]) -> Result<()> {
    for path in outputs {
        let linked = path
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);
        if linked || (path.exists() && !path.is_file()) {
            return invalid(format!(
                "output target must be a regular file: {}",
                path.display()
            ));
        }
        let parent = path.parent().unwrap_or(Path::new(""));
        let parent_linked = parent
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);
        if parent != directory && parent_linked {
            return invalid(format!(
                "output parent must not be a symbolic link: {}",
                parent.display()
            ));
        }
        fs::create_dir_all(parent).map_err(|_| {
            Error::InvalidInput(format!("invalid output parent: {}", parent.display()))
        })?;
    }
    Ok(())
}

/// Refuse or admit replacement, including detached input payloads
/// (`check_output_replacement`).
pub fn check_output_replacement(
    existing: &[String],
    overwrite: bool,
    inputs: &[PathBuf],
) -> Result<()> {
    if existing.is_empty() {
        return Ok(());
    }
    if !overwrite {
        return invalid(format!(
            "outputs already exist: {}; pass overwrite=True to replace them",
            existing.join(", ")
        ));
    }
    // A detached image can reference output files or their hard links. Refuse all
    // replacement rather than guessing every possible payload path.
    for source in inputs {
        if has_detached_data(source)? {
            return invalid(
                "cannot replace existing outputs with detached NRRD, NIfTI or Analyze inputs; \
                 choose a new output directory",
            );
        }
    }
    Ok(())
}

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn random_token() -> String {
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789_";
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let mut state = nanos
        ^ (u64::from(std::process::id()) << 32)
        ^ COUNTER
            .fetch_add(1, Ordering::Relaxed)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    (0..8)
        .map(|_| {
            // splitmix64 step
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^= z >> 31;
            ALPHABET[(z % ALPHABET.len() as u64) as usize] as char
        })
        .collect()
}

/// `Path.suffixes` joined: everything from the first dot after any leading dots.
fn joined_suffixes(target: &Path) -> String {
    let name = target
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let trimmed = name.trim_start_matches('.');
    if trimmed.ends_with('.') {
        return String::new();
    }
    trimmed
        .find('.')
        .map(|i| trimmed[i..].to_string())
        .unwrap_or_default()
}

/// Exclusive staging files in a private directory on the output filesystem, with
/// restoration of replaced outputs if publication fails (`Publication`).
#[derive(Debug)]
pub struct Publication {
    root: PathBuf,
    staged: Vec<(PathBuf, PathBuf)>,
    retain_backups: bool,
    cleaned: bool,
}

impl Publication {
    /// Create a marked private staging directory inside `directory`.
    pub fn create(directory: &Path) -> Result<Self> {
        let mut attempts = 0;
        let root = loop {
            let candidate = directory.join(format!("{STAGING_PREFIX}{}", random_token()));
            let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            match builder.create(&candidate) {
                Ok(()) => break candidate,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists && attempts < 100 => {
                    attempts += 1;
                }
                Err(e) => return Err(e.into()),
            }
        };
        fs::write(root.join(MARKER), b"1\n")?;
        Ok(Self {
            root,
            staged: Vec::new(),
            retain_backups: false,
            cleaned: false,
        })
    }

    /// Reserve a private file with the target's suffixes, for format-selecting writers.
    pub fn stage(&mut self, target: &Path) -> Result<PathBuf> {
        let suffix = joined_suffixes(target);
        let mut attempts = 0;
        loop {
            let candidate = self.root.join(format!("{}{suffix}", random_token()));
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&candidate)
            {
                Ok(_) => {
                    self.staged.retain(|(t, _)| t != target);
                    self.staged.push((target.to_path_buf(), candidate.clone()));
                    return Ok(candidate);
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists && attempts < 100 => {
                    attempts += 1;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    /// Publish or restore a generation, then clean staging.
    ///
    /// With `overwrite`, each existing target keeps its old inode as a hard-linked backup
    /// until every target has been replaced; any failure restores the targets in reverse
    /// order. Without it each target is a no-clobber hard link.
    pub fn publish(&mut self, overwrite: bool) -> Result<()> {
        let result = self.publish_inner(overwrite);
        self.cleanup();
        result
    }

    fn publish_inner(&mut self, overwrite: bool) -> Result<()> {
        // Keep recovery copies if rollback is interrupted.
        self.retain_backups = true;
        let mut backups: Vec<(PathBuf, PathBuf)> = Vec::new();
        let mut attempted: Vec<(PathBuf, PathBuf)> = Vec::new();
        let outcome = (|| -> Result<()> {
            if overwrite {
                for (target, _) in &self.staged {
                    if target.symlink_metadata().is_ok() {
                        let name = target
                            .file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        let backup = self.root.join(format!("backup-{name}"));
                        // Preserve the old inode without copying full voxel maps.
                        fs::hard_link(target, &backup)?;
                        backups.push((target.clone(), backup));
                    }
                }
            }
            for (target, temporary) in &self.staged {
                attempted.push((target.clone(), temporary.clone()));
                if overwrite {
                    fs::rename(temporary, target)?;
                } else if let Err(error) = fs::hard_link(temporary, target) {
                    if error.kind() == std::io::ErrorKind::AlreadyExists {
                        return invalid(format!(
                            "output already exists: {}; pass overwrite=True to replace it",
                            target.display()
                        ));
                    }
                    return Err(error.into());
                }
            }
            Ok(())
        })();
        if outcome.is_ok() {
            self.retain_backups = false;
            return Ok(());
        }
        let mut failure: Option<std::io::Error> = None;
        for (target, staged) in attempted.iter().rev() {
            let backup = backups.iter().find(|(t, _)| t == target).map(|(_, b)| b);
            let restored = (|| -> std::io::Result<()> {
                #[cfg(unix)]
                let owned = {
                    use std::os::unix::fs::MetadataExt;
                    match (target.symlink_metadata(), staged.symlink_metadata()) {
                        (Ok(now), Ok(was)) => {
                            Some(now.ino() == was.ino() && now.dev() == was.dev())
                        }
                        // The staged file moved here by rename, or the target is gone.
                        (Ok(_), Err(_)) => Some(true),
                        (Err(_), _) => None,
                    }
                };
                #[cfg(not(unix))]
                let owned = target.symlink_metadata().ok().map(|_| true);
                match owned {
                    None => {
                        if let Some(backup) = backup {
                            fs::hard_link(backup, target)?;
                        }
                    }
                    Some(false) => {
                        // A failed no-clobber link must preserve another writer's file.
                        // Only a changed file with no backup to compare is left alone.
                    }
                    Some(true) => match backup {
                        Some(backup) => fs::rename(backup, target)?,
                        None => fs::remove_file(target)?,
                    },
                }
                Ok(())
            })();
            if let Err(error) = restored {
                failure.get_or_insert(error);
            }
        }
        if failure.is_some() {
            // Previous files stay at the staging root; cleanup keeps it.
            return Err(Error::Execution(format!(
                "cannot restore outputs; previous files retained at {}",
                self.root.display()
            )));
        }
        self.retain_backups = false;
        outcome
    }

    /// Clean owned staging once; retain recovery copies if rollback failed.
    pub fn cleanup(&mut self) {
        if !self.cleaned && !self.retain_backups {
            let _ = fs::remove_dir_all(&self.root);
            self.cleaned = true;
        }
    }
}

impl Drop for Publication {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Remove this library's abandoned staging after the directory lock is held
/// (`reclaim_stale_staging`).
///
/// A killed process cannot run cleanup. Its marked staging directory is reclaimed by the
/// next run. Directories containing a `backup-*` recovery copy are always retained, empty
/// name-matching directories are removed, and unrecognized nonempty directories are kept.
/// Reclamation is best effort.
pub fn reclaim_stale_staging(directory: &Path) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let token = name.strip_prefix(STAGING_PREFIX).unwrap_or("");
        let matches_name = token.len() == 8
            && token
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        let is_link = path
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(true);
        if !matches_name || is_link || !path.is_dir() {
            continue;
        }
        let Ok(contents) = fs::read_dir(&path) else {
            continue;
        };
        let names: Vec<String> = contents
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        if names.iter().any(|n| n.starts_with("backup-")) {
            continue;
        }
        if names.is_empty() || names.iter().any(|n| n == MARKER) {
            let _ = fs::remove_dir_all(&path);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workflows::publication::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("radiomics-w8-{name}-{}", random_token()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn suffixes_follow_pathlib() {
        assert_eq!(joined_suffixes(Path::new("a/map.nii.gz")), ".nii.gz");
        assert_eq!(joined_suffixes(Path::new("a/.hidden")), "");
        assert_eq!(joined_suffixes(Path::new("a/x.b.nii.gz")), ".b.nii.gz");
        assert_eq!(joined_suffixes(Path::new("a/x.")), "");
    }

    #[test]
    fn stale_staging_is_reclaimed_unless_it_holds_a_backup() {
        let dir = scratch("stale");
        let stale = dir.join(".mlxradiomics-abcd0123");
        let kept = dir.join(".mlxradiomics-wxyz4567");
        let foreign = dir.join(".mlxradiomics-unknown1");
        for path in [&stale, &kept, &foreign] {
            fs::create_dir(path).unwrap();
        }
        fs::write(stale.join(MARKER), "1\n").unwrap();
        fs::write(kept.join(MARKER), "1\n").unwrap();
        fs::write(kept.join("backup-features.json"), "old").unwrap();
        fs::write(foreign.join("note"), "x").unwrap();
        reclaim_stale_staging(&dir);
        assert!(!stale.exists());
        assert!(kept.exists());
        assert!(foreign.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn lock_is_exclusive_and_released_on_drop() {
        let dir = scratch("lock");
        let held = output_lock(&dir).unwrap();
        let error = output_lock(&dir).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("another run is using output directory")
        );
        drop(held);
        output_lock(&dir).unwrap();
        fs::remove_dir_all(dir).unwrap();
    }
}
