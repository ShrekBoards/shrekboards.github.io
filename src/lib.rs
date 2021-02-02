use std::collections::HashMap;

use shrek_superslam::{Console, MasterDat, MasterDir};
use shrek_superslam::classes::attacks::AttackMoveType;
use shrek_superslam::files::Bin;
use wasm_bindgen::prelude::*;

/// Extract the character attacks from the passed MASTER.DAT and MASTER.DIR
/// files from a game on the given `console`, and return the attacks as a
/// JSON object.
#[wasm_bindgen]
pub fn entry1(master_dat: &[u8], master_dir: &[u8], console: i32) -> JsValue {
    console_error_panic_hook::set_once();
    let console = match console {
        0 => Console::Gamecube,
        1 => Console::PC,
        2 => Console::PS2,
        3 => Console::Xbox,
        _ => panic!("uh oh")
    };

    let master_dir = MasterDir::from_bytes(master_dir, console);
    let master_dat = MasterDat::from_bytes(master_dat, master_dir);
    JsValue::from_serde(&parse_attacks(&master_dat, console)).unwrap()
}

/// Get all character Game::AttackMoveType objects from the player objects
/// present in the passed MASTER.DAT file for the given `console`, and return
/// them as a dictionary mapping.
fn parse_attacks(master_dat: &MasterDat, console: Console) -> HashMap::<String, Vec<AttackMoveType>> {
    let mut attacks = HashMap::<String, Vec<AttackMoveType>>::new();

    // Enumerate all files to find the player.db.bin files
    for filepath in master_dat.files() {
        // Get the filename of
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