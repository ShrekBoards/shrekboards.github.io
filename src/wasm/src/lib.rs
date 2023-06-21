mod characters;
mod game_files;
mod stages;
mod util;

use js_sys::{Array, Uint8Array};
use shrek_superslam::{MasterDat, MasterDir};
use wasm_bindgen::prelude::*;

use characters::{CharacterCollection, insert_character_collection, new_character_collection};
use stages::{insert_stage_collection, new_stage_collection, StageCollection};
use util::console_from_value;

/// Extract the game characters from the passed MASTER.DAT and MASTER.DIR
/// files from a game on the given `console`, and return the characters as a
/// JSON object.
#[wasm_bindgen]
pub fn extract_game_characters(master_dat_bytes: &[u8], master_dir_bytes: &[u8], console_num: i32) -> JsValue {
    console_error_panic_hook::set_once();

    let console = console_from_value(console_num);
    let master_dir = MasterDir::from_bytes(master_dir_bytes, console).unwrap();
    let master_dat = MasterDat::from_bytes(master_dat_bytes, master_dir);
    JsValue::from_serde(&new_character_collection(&master_dat, console)).unwrap()
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
    JsValue::from_serde(&new_stage_collection(&master_dat, console)).unwrap()
}

/// Given the characters and stages as JSON objects, overwrite the original
/// values in the passed MASTER.DAT and MASTER.DIR from the given `console`
/// with the new details from the JSON object.
///
/// Returns a JavaScript array containing two Uint8Arrays for the bytes of the
/// recomputed MASTER.DIR and MASTER.DAT, in that order.
#[wasm_bindgen]
pub fn recreate_game_files(master_dat_bytes: &[u8], master_dir_bytes: &[u8], console_num: i32, characters_json: JsValue, stages_json: JsValue) -> Array {
    console_error_panic_hook::set_once();

    let console = console_from_value(console_num);
    let master_dir = MasterDir::from_bytes(master_dir_bytes, console).unwrap();
    let mut master_dat = MasterDat::from_bytes(master_dat_bytes, master_dir);

    let characters: CharacterCollection = characters_json.into_serde().unwrap();
    insert_character_collection(&characters, &mut master_dat, console);

    let stages: StageCollection = stages_json.into_serde().unwrap();
    insert_stage_collection(&stages, &mut master_dat, console);

    let (new_master_dat_bytes, new_master_dir_bytes) = master_dat.to_bytes().unwrap();
    vec!(new_master_dat_bytes, new_master_dir_bytes).iter().map(|x| Uint8Array::from(x.as_slice())).collect()
}
