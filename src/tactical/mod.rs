
use std::{fs::read_dir, path::PathBuf};

use crate::tactical::xmltypes::MarkerPack;

pub mod localtypes;
pub mod scene;
pub mod xmltypes;
// pub struct MarkerManager {
//     pub mar_cats: Vec<MarCat>,
//     pub scene: MarkerScene,
// }
// impl MarkerManager {
//     pub fn new(gl: Rc<Context>, marker_path: &str) -> Self {
//         MarkerManager {
//             mar_cats: load_markers(marker_path).unwrap(),
//             scene: MarkerScene::new(gl),
//         }
//     }
//     pub fn load_markers(&mut self, marker_path: &str) {
//         self.mar_cats = load_markers(marker_path).unwrap();
//     }
//     pub fn get_present_map_markers_with_inherit(
//         marker_cats: &Vec<MarCat>,
//         map_id: u32,
//         current_enabled_map_markers: &mut Vec<XMLMarker>,
//         prev_template: &XMLCategory,
//     ) {
//         for mc in marker_cats.iter() {
//             if mc.enabled {
//                 let mut current_template = prev_template.clone();
//                 current_template.inherit_if_none(&mc.xml_cat);
//                 for m in &mc.markers {
//                     if m.map_id == map_id {
//                         let mut marker = m.clone();
//                         marker.inherit_if_none(&current_template);
//                         current_enabled_map_markers.push(marker);
//                     }
//                 }
//                 MarkerManager::get_present_map_markers_with_inherit(
//                     &mc.children,
//                     map_id,
//                     current_enabled_map_markers,
//                     &current_template,
//                 );
//             }
//         }
//     }
//     pub fn update_scene_markers_to_current_map(&mut self, map_id: u32) {
//         let mut markers: Vec<XMLMarker> = vec![];
//         MarkerManager::get_present_map_markers_with_inherit(
//             &self.mar_cats,
//             map_id,
//             &mut markers,
//             &XMLCategory::default(),
//         );
//         self.scene
//             .update_marker_nodes(&markers)
//             .map_err(|e| {
//                 log::error!("failed to update scene nodes from markers: {}", &e);
//                 e
//             })
//             .unwrap();
//     }

// }
pub struct MarkerManager {
    pub marker_packs: Vec<MarkerPack>,
    pub location: PathBuf,
}

impl MarkerManager {
    pub fn new(location: &str) -> MarkerManager {
        let mut marker_packs = Vec::new();
        let entries = read_dir(location)
            .map_err(|e| {
                log::error!("couldnt' read dir entries. error: {:?}", &e);
                e
            })
            .unwrap();
        for entry in entries {
            let entry = entry
                .map_err(|e| {
                    log::error!("{:?}", &e);
                    e
                })
                .unwrap();
            if entry
                .metadata()
                .map_err(|e| {
                    log::error!("{:?}", &e);
                })
                .unwrap()
                .is_dir()
            {
                marker_packs.push(MarkerPack::new(&entry.path()));
            }
        }
        MarkerManager {
            marker_packs,
            location: location.into(),
        }
    }
}
