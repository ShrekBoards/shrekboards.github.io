use std::collections::HashMap;

use serde::Serialize;
use shrek_superslam::{Console, MasterDat};
use shrek_superslam::classes::{GfDb, SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use shrek_superslam::files::Bin;

/// Get all paths in the MASTER.DAT to the requested filename.
pub fn get_all_paths_with_filename(master_dat: &MasterDat, requested_filename: &'static str) -> Vec<String> {
    let mut found_filepaths: Vec<String> = vec![];
    
    for filepath in master_dat.files() {
        // Due to an old bug in the Rust library repackage code, some repackaged
        // versions use forward slashes rather than backslashes, so try and
        // accomodate those.
        //
        // <https://github.com/ShrekBoards/shrek-superslam/commit/0fb54ebd882f6eb780201bf0f0172750d7af4e05>
        let mut iter = if filepath.contains('/') && !filepath.contains('\\') {
            filepath.rsplit('/').take(2)
        } else {
            filepath.rsplit('\\').take(2)
        };
        let filename = iter.next().unwrap();

        if filename == requested_filename {
            found_filepaths.push(filepath);
        }
    }

    found_filepaths
}

/// Get the parent directory of the given filepath.
pub fn parent_of_filepath(filepath: &str) -> String {
    let iter = if filepath.contains('/') && !filepath.contains('\\') {
        filepath.rsplit('/').take(2)
    } else {
        filepath.rsplit('\\').take(2)
    };

    iter.skip(1).next().unwrap().to_owned()
}

/// Get all objects of the given type parameter from the given filepath within
/// the given MASTER.DAT.
pub fn get_all_objects_of_type_from_file<T>(filepath: &str, master_dat: &MasterDat, console: Console) -> HashMap<String, T>
    where T: SerialisedShrekSuperSlamGameObject + Serialize
{
    // Read the found file, extract the gf::DB index object
    let bin = Bin::new(master_dat.decompressed_file(&filepath).unwrap(), console).unwrap();
    let db = bin.get_object_from_offset::<GfDb>(0x00).unwrap();

    // Resolve all objects of the requested type, and correlate them with
    // their DB name
    db.entries
        .into_iter()
        .filter_map(|(db_entry_name, obj)| if obj.hash == T::hash() { Some((db_entry_name, bin.get_object_from_offset::<T>(obj.offset).unwrap())) } else { None })
        .collect()
}

/// Insert the given objects into the given file within the given MASTER.DAT file.
pub fn insert_updated_objects<T>(
    master_dat: &mut MasterDat,
    console: Console,
    filepath: &str,
    all_objects: &HashMap<String, T>
)
    where T: SerialisedShrekSuperSlamGameObject + WriteableShrekSuperSlamGameObject
{
    // Read the passed file from the MASTER.DAT.
    let decompressed_file = master_dat.decompressed_file(&filepath).unwrap();
    let mut bin = Bin::new(decompressed_file, console).unwrap();

    // Read the index gf::DB object in this .bin file.
    let db = bin.get_object_from_offset::<GfDb>(0x00).unwrap();

    // Replace each object in the DB that we have a replacement for.
    for (db_entry_name, obj) in &db.entries {
        if let Some(replacement_object) = all_objects.get(db_entry_name) {
            if bin.overwrite_object(obj.offset, replacement_object).is_err() {
                panic!(
                    "error overwriting object '{}' in '{}'",
                    db_entry_name, filepath
                );
            }
        }
    }

    // Write the updated .bin file to the MASTER.DAT
    if let Err(i) = master_dat.update_file(&filepath, bin.raw()) {
        panic!(
            "updated file '{}' had wrong size: {} instead of {}",
            filepath,
            bin.raw().len(),
            i
        );
    }
}