use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use shrek_superslam::{Console, MasterDat};
use shrek_superslam::classes::{AttackMoveType, PhysicsFighting};

use crate::game_files::*;

// Properties of a single playable character within Shrek SuperSlam.
#[derive(Deserialize, Serialize)]
pub struct Character {
    /// The attacks of the character, keyed on the attack name.
    pub attacks: HashMap<String, AttackMoveType>,

    /// The character's physics.
    pub physics: HashMap<String, PhysicsFighting>,
}

impl Character {
    pub fn new(player_db_bin_path: &str, master_dat: &MasterDat, console: Console) -> Character {
        Character {
            attacks: get_all_objects_of_type_from_file::<AttackMoveType>(player_db_bin_path, master_dat, console),
            physics: get_all_objects_of_type_from_file::<PhysicsFighting>(player_db_bin_path, master_dat, console),
        }
    }
}

/// Collection of all characters within Shrek SuperSlam.
pub type CharacterCollection = HashMap<String, Character>;

/// Create a new CharacterCollection from the passed MASTER.DAT.
pub fn new_character_collection(master_dat: &MasterDat, console: Console) -> CharacterCollection {
    get_all_paths_with_filename(master_dat, "player.db.bin")
                                .into_iter()
                                .map(|path| (parent_of_filepath(&path), Character::new(&path, master_dat, console)))
                                .collect()
}

/// Insert the passed CharacterCollection into the passed MASTER.DAT.
pub fn insert_character_collection(characters: &CharacterCollection, master_dat: &mut MasterDat, console: Console) {
    for (character_name, character_obj) in characters {
        let filepath = format!("data\\players\\{}\\player.db.bin", character_name);
        insert_updated_objects::<AttackMoveType>(master_dat, console, &filepath, &character_obj.attacks);
        insert_updated_objects::<PhysicsFighting>(master_dat, console, &filepath, &character_obj.physics);
    }
}