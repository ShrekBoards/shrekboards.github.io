use std::collections::HashMap;

use js_sys::{Array, Uint8Array};
use shrek_superslam::{Console, MasterDat, MasterDir};
use shrek_superslam::classes::attacks::AttackMoveType;
use shrek_superslam::files::Bin;
use wasm_bindgen::prelude::*;

/// Extract the character attacks from the passed MASTER.DAT and MASTER.DIR
/// files from a game on the given `console`, and return the attacks as a
/// JSON object.
#[wasm_bindgen]
pub fn extract_character_attacks(master_dat_bytes: &[u8], master_dir_bytes: &[u8], console_num: i32) -> JsValue {
    console_error_panic_hook::set_once();

    let console = console_from_value(console_num);
    let master_dir = MasterDir::from_bytes(master_dir_bytes, console);
    let master_dat = MasterDat::from_bytes(master_dat_bytes, master_dir);
    JsValue::from_serde(&parse_attacks(&master_dat, console)).unwrap()
}

/// Given the character `attacks` as a JSON object, overwrite the character
/// attacks in the passed MASTER.DAT and MASTER.DIR from the given `console`
/// with the new details from the JSON object.
///
/// Returns a JavaScript array containing two Uint8Arrays for the bytes of the
/// recomputed MASTER.DIR and MASTER.DAT, in that order.
#[wasm_bindgen]
pub fn recreate_game_files(master_dat_bytes: &[u8], master_dir_bytes: &[u8], console_num: i32, character_attacks: JsValue) -> Array {
    console_error_panic_hook::set_once();

    let console = console_from_value(console_num);
    let master_dir = MasterDir::from_bytes(master_dir_bytes, console);
    let mut master_dat = MasterDat::from_bytes(master_dat_bytes, master_dir);
    let attacks: HashMap<String, Vec<AttackMoveType>> = character_attacks.into_serde().unwrap();
    let (new_master_dat_bytes, new_master_dir_bytes) = insert_new_attacks(&mut master_dat, console, &attacks);
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
fn parse_attacks(master_dat: &MasterDat, console: Console) -> HashMap::<String, Vec<AttackMoveType>> {
    let mut attacks = HashMap::<String, Vec<AttackMoveType>>::new();

    // Enumerate all files in the MASTER.DAT to find the player.db.bin files
    for filepath in master_dat.files() {
        let mut iter = filepath.rsplit('\\').take(2);
        let filename = iter.next().unwrap();

        if filename == "player.db.bin" {
            // Get the character name from the directory containing the file
            let character = iter.next().unwrap();

            // Read the player.db.bin file, grab all the Game::AttackMoveType
            // objects and convert them to JSON objects
            let bin = Bin::new(master_dat.decompressed_file(&filepath).unwrap(), console);
            let objects = bin
                .get_all_objects_of_type::<AttackMoveType>()
                .into_iter()
                .map(|(_, a)| a)
                .collect();

            attacks.insert(character.to_owned(), objects);
        }
    }

    attacks
}

/// Insert the given `attacks` into the given MASTER.DAT file for the given `console`.
fn insert_new_attacks(master_dat: &mut MasterDat, console: Console, attacks: &HashMap<String, Vec<AttackMoveType>>) -> (Vec<u8>, Vec<u8>) {
    for (character, attacks) in attacks {
        // Read the player.db.bin file for this character
        let filename = format!("data\\players\\{}\\player.db.bin", character);
        let mut bin = Bin::new(master_dat.decompressed_file(&filename).unwrap(), console);

        // Collect every Game::AttackMoveType object in the player.db.bin file,
        // along with the attack's offset within the file
        let original_attacks = bin.get_all_objects_of_type::<AttackMoveType>();

        // Iterate over each of these attacks, find the matching attack in the
        // updated JSON file, then overwrite the attack in the player.db.bin
        // file with the updated one from the JSON
        for (offset, attack) in original_attacks {
            let matching_json_attack = match attacks.iter().find(|a| a.name == attack.name) {
                Some(a) => a,
                _ => panic!("Could not find attack '{}' in '{}'", attack.name, filename),
            };
            if bin.overwrite_object(offset, matching_json_attack).is_err() {
                panic!(
                    "error overwriting attack '{}' in '{}'",
                    attack.name, filename
                );
            }
        }

        // Write the updated .bin file to the MASTER.DAT
        if let Err(i) = master_dat.update_file(&filename, bin.raw()) {
            panic!(
                "updated file had wrong size: {} instead of {}",
                bin.raw().len(),
                i
            );
        }
    }
    master_dat.to_bytes()
}