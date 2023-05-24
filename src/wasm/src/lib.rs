use std::collections::HashMap;

use js_sys::{Array, Uint8Array};
use serde::Serialize;
use shrek_superslam::{Console, MasterDat, MasterDir};
use shrek_superslam::classes::{AttackMoveType, GfDb, SerialisedShrekSuperSlamGameObject, Spitter, WriteableShrekSuperSlamGameObject};
use shrek_superslam::files::Bin;
use wasm_bindgen::prelude::*;

/// Extract the character attacks from the passed MASTER.DAT and MASTER.DIR
/// files from a game on the given `console`, and return the attacks as a
/// JSON object.
#[wasm_bindgen]
pub fn extract_character_attacks(master_dat_bytes: &[u8], master_dir_bytes: &[u8], console_num: i32) -> JsValue {
    console_error_panic_hook::set_once();

    let console = console_from_value(console_num);
    let master_dir = MasterDir::from_bytes(master_dir_bytes, console).unwrap();
    let master_dat = MasterDat::from_bytes(master_dat_bytes, master_dir);
    JsValue::from_serde(&parse_attacks(&master_dat, console)).unwrap()
}

/// Extract the game stages from the passed MASTER.DAT and MASTER.DIR
/// files from a game on the given `console`, and return the stages as a
/// JSON object.
#[wasm_bindgen]
pub fn extract_game_stages(master_dat_bytes: &[u8], master_dir_bytes: &[u8], console_num: i32) -> JsValue {
    console_error_panic_hook::set_once();

    let console = console_from_value(console_num);
    let master_dir = MasterDir::from_bytes(master_dir_bytes, console).unwrap();
    let master_dat = MasterDat::from_bytes(master_dat_bytes, master_dir);
    JsValue::from_serde(&parse_spitters(&master_dat, console)).unwrap()
}

/// Given the character `attacks` as a JSON object, overwrite the character
/// attacks in the passed MASTER.DAT and MASTER.DIR from the given `console`
/// with the new details from the JSON object.
///
/// Returns a JavaScript array containing two Uint8Arrays for the bytes of the
/// recomputed MASTER.DIR and MASTER.DAT, in that order.
#[wasm_bindgen]
pub fn recreate_game_files(master_dat_bytes: &[u8], master_dir_bytes: &[u8], console_num: i32, character_attacks: JsValue, stage_values: JsValue) -> Array {
    console_error_panic_hook::set_once();

    let console = console_from_value(console_num);
    let master_dir = MasterDir::from_bytes(master_dir_bytes, console).unwrap();
    let mut master_dat = MasterDat::from_bytes(master_dat_bytes, master_dir);
    let attacks: HashMap<String, HashMap<String, AttackMoveType>> = character_attacks.into_serde().unwrap();
    insert_new_attacks(&mut master_dat, console, &attacks);
    let stages: HashMap<String, HashMap<String, Spitter>> = stage_values.into_serde().unwrap();
    insert_new_spitters(&mut master_dat, console, &stages);
    let (new_master_dat_bytes, new_master_dir_bytes) = master_dat.to_bytes().unwrap();
    vec!(new_master_dat_bytes, new_master_dir_bytes).iter().map(|x| Uint8Array::from(x.as_slice())).collect()
}

/// Turn an integer value from the JS into a Console enum.
fn console_from_value(console: i32) -> Console {
    match console {
        0 => Console::Gamecube,
        1 => Console::PC,
        2 => Console::PS2,
        3 => Console::Xbox,
        _ => panic!("uh oh")
    }
}

/// Get all character Game::AttackMoveType objects from the player objects
/// present in the passed MASTER.DAT file for the given `console`, and return
/// them as a dictionary mapping.
fn parse_attacks(master_dat: &MasterDat, console: Console) -> HashMap::<String, HashMap<String, AttackMoveType>> {
    parse_objects::<AttackMoveType>(master_dat, console, "player.db.bin")
}

/// Get all character Game::Spitter objects from the player objects
/// present in the passed MASTER.DAT file for the given `console`, and return
/// them as a dictionary mapping.
fn parse_spitters(master_dat: &MasterDat, console: Console) -> HashMap::<String, HashMap<String, Spitter>> {
    parse_objects::<Spitter>(master_dat, console, "level.db.bin")
}

/// Insert a collection of updated attack objects to the passed MASTER.DAT.
fn insert_new_attacks(master_dat: &mut MasterDat, console: Console, objects: &HashMap<String, HashMap<String, AttackMoveType>>) {
    insert_updated_objects::<AttackMoveType>(master_dat, console, "players", "player.db.bin", objects)
}

/// Insert a collection of updated spitter objects to the passed MASTER.DAT.
fn insert_new_spitters(master_dat: &mut MasterDat, console: Console, objects: &HashMap<String, HashMap<String, Spitter>>) {
    insert_updated_objects::<Spitter>(master_dat, console, "levels", "level.db.bin", objects)
}

/// Generic method to grab all copies of a specified game type from all
/// files with a given name in the MASTER.DAT for the given console.
fn parse_objects<T>(master_dat: &MasterDat, console: Console, expected_filename: &'static str) -> HashMap::<String, HashMap<String, T>>
    where T: SerialisedShrekSuperSlamGameObject + Serialize
{
    let mut objects: HashMap<String, HashMap<String, T>> = HashMap::new();

    // Enumerate all files in the MASTER.DAT to find the requested files
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

        if filename == expected_filename {
            // Get the character or level name from the directory containing the file
            let name = iter.next().unwrap();

            // Read the found file, extract the gf::DB index object
            let bin = Bin::new(master_dat.decompressed_file(&filepath).unwrap(), console).unwrap();
            let db = bin.get_object_from_offset::<GfDb>(0x00).unwrap();

            // Resolve all objects of the requested type, and correlate them with
            // their DB name
            let resolved_objects_in_file: HashMap<String, T> = db.entries
                .into_iter()
                .filter_map(|(db_entry_name, obj)| if obj.hash == T::hash() { Some((db_entry_name, bin.get_object_from_offset::<T>(obj.offset).unwrap())) } else { None })
                .collect();
 
            objects.insert(name.to_owned(), resolved_objects_in_file);
        }
    }

    objects
}

/// Insert the given objects into the given MASTER.DAT file for the given `console`.
fn insert_updated_objects<T>(
    master_dat: &mut MasterDat,
    console: Console,
    dir_name: &'static str,
    expected_filename: &'static str,
    all_objects: &HashMap<String, HashMap<String, T>>
)
    where T: SerialisedShrekSuperSlamGameObject + WriteableShrekSuperSlamGameObject
{
    for (name, objects) in all_objects {
        // Determine the correct filename. Due to a bug in older versions of
        // the repackaging code in the shrek-superslam crate, the filename
        // may use forward slashes. We use compressed_file to check for the
        // existence of the file because its quicker.
        let filename1 = format!("data\\{}\\{}\\{}", dir_name, name, expected_filename);
        let filename2 = format!("data/{}/{}/{}", dir_name, name, expected_filename);
        let filename = if master_dat.compressed_file(&filename1).is_some() {
            filename1
        } else if master_dat.compressed_file(&filename2).is_some() {
            filename2
        } else {
            panic!("Could not find file '{}' in MASTER.DAT", filename1);
        };

        // Read the player.db.bin file for this character
        let decompressed_file = master_dat.decompressed_file(&filename).unwrap();
        let mut bin = Bin::new(decompressed_file, console).unwrap();

        // Read the index gf::DB object in this .bin file.
        let db = bin.get_object_from_offset::<GfDb>(0x00).unwrap();

        // Replace each object in the DB that we have a replacement for.
        for (db_entry_name, obj) in &db.entries {
            if let Some(replacement_object) = objects.get(db_entry_name) {
                if bin.overwrite_object(obj.offset, replacement_object).is_err() {
                    panic!(
                        "error overwriting object '{}' in '{}'",
                        db_entry_name, filename
                    );
                }
            }
        }

        // Write the updated .bin file to the MASTER.DAT
        if let Err(i) = master_dat.update_file(&filename, bin.raw()) {
            panic!(
                "updated file '{}' had wrong size: {} instead of {}",
                filename,
                bin.raw().len(),
                i
            );
        }
    }
}