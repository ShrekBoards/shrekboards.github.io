use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use shrek_superslam::{Console, MasterDat};
use shrek_superslam::classes::Spitter;

use crate::game_files::*;

// Properties of a single stage within Shrek SuperSlam.
#[derive(Deserialize, Serialize)]
pub struct Stage {
    /// The spitters of the stage, keyed on the spitter name.
    pub spitters: HashMap<String, Spitter>,
}

impl Stage {
    pub fn new(player_db_bin_path: &str, master_dat: &MasterDat, console: Console) -> Stage {
        Stage {
            spitters: get_all_objects_of_type_from_file::<Spitter>(player_db_bin_path, master_dat, console),
        }
    }
}

/// Collection of all stages within Shrek SuperSlam.
pub type StageCollection = HashMap<String, Stage>;

/// Create a new StageCollection from the passed MASTER.DAT.
pub fn new_stage_collection(master_dat: &MasterDat, console: Console) -> StageCollection {
    get_all_paths_with_filename(master_dat, "level.db.bin")
                                .into_iter()
                                .map(|path| (parent_of_filepath(&path), Stage::new(&path, master_dat, console)))
                                .collect()
}

/// Insert the passed StageCollection into the passed MASTER.DAT.
pub fn insert_stage_collection(stages: &StageCollection, master_dat: &mut MasterDat, console: Console) {
    for (stage_name, stage_obj) in stages {
        let filepath = format!("data\\levels\\{}\\level.db.bin", stage_name);
        insert_updated_objects::<Spitter>(master_dat, console, &filepath, &stage_obj.spitters);
    }
}