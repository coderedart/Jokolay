use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet}, sync::{Arc, Mutex}
};

use glam::Vec3;
use tribool::Tribool;
use cap_std::fs_utf8::Dir;
use egui::{CollapsingHeader, ColorImage, TextureHandle, Window};
use image::EncodableLayout;

use tracing::{error, info, info_span, trace};

use joko_core::RelativePath;
use jokolink::MumbleLink;
use miette::{Context, IntoDiagnostic, Result};
use uuid::Uuid;
use crate::message::{UIToBackMessage, UIToUIMessage};

use crate::{message::BackToUIMessage, pack::CommonAttributes};
use crate::manager::pack::loaded::{LoadedPackData, PackTasks, LoadedPackTexture};
use crate::manager::pack::import::{ImportStatus, import_pack_from_zip_file_path};

use super::pack::loaded::jokolay_to_marker_dir;

pub const PACKAGE_MANAGER_DIRECTORY_NAME: &str = "marker_manager";//name kept for compatibility purpose
pub const PACKAGES_DIRECTORY_NAME: &str = "packs";//name kept for compatibility purpose
// pub const MARKER_MANAGER_CONFIG_NAME: &str = "marker_manager_config.json";

/// It manage everything that has to do with marker packs.
/// 1. imports, loads, saves and exports marker packs.
/// 2. maintains the categories selection data for every pack
/// 3. contains activation data globally and per character
/// 4. When we load into a map, it filters the markers and runs the logic every frame
///     1. If a marker needs to be activated (based on player position or whatever)
///     2. marker needs to be drawn
///     3. marker's texture is uploaded or being uploaded? if not ready, we will upload or use a temporary "loading" texture
///     4. render that marker use joko_render  
/// FIXME: it is a bad name, it does not manage Markers, but packages
#[must_use]
pub struct PackageDataManager {
    /// marker manager directory. not useful yet, but in future we could be using this to store config files etc..
    //_marker_manager_dir: Arc<Dir>,
    /// packs directory which contains marker packs. each directory inside pack directory is an individual marker pack.
    /// The name of the child directory is the name of the pack
    pub marker_packs_dir: Arc<Dir>,
    /// These are the marker packs
    /// The key is the name of the pack
    /// The value is a loaded pack that contains additional data for live marker packs like what needs to be saved or category selections etc..
    pub packs: BTreeMap<Uuid, LoadedPackData>,
    tasks: PackTasks,
    current_map_id: u32,
    show_only_active: bool,
    /// This is the interval in number of seconds when we check if any of the packs need to be saved due to changes.
    /// This allows us to avoid saving the pack too often.
    pub save_interval: f64,

    pub currently_used_files: BTreeMap<String, bool>,
    parents: HashMap<Uuid, Uuid>,
    loaded_elements: HashSet<Uuid>,
    on_screen: BTreeSet<Uuid>,
}
#[must_use]
pub struct PackageUIManager {
    pub import_status: Option<Arc<Mutex<ImportStatus>>>,
    default_marker_texture: Option<TextureHandle>,
    default_trail_texture: Option<TextureHandle>,
    packs: BTreeMap<Uuid, LoadedPackTexture>,
    tasks: PackTasks,

    currently_used_files: BTreeMap<String, bool>,
    all_files_tribool: Tribool,
    all_files_toggle: bool,
    show_only_active: bool,
}

impl PackageDataManager {
    /// Creates a new instance of [MarkerManager].
    /// 1. It opens the marker manager directory
    /// 2. loads its configuration
    /// 3. opens the packs directory
    /// 4. loads all the packs
    /// 5. loads all the activation data
    /// 6. returns self
    pub fn new(packs: BTreeMap<Uuid, LoadedPackData>, jokolay_dir: Arc<Dir>) -> Result<Self> {
        let marker_packs_dir = jokolay_to_marker_dir(&jokolay_dir)?;
        Ok(Self {
            packs,
            tasks: PackTasks::new(),
            marker_packs_dir: Arc::new(marker_packs_dir),
            //_marker_manager_dir: marker_manager_dir.into(),
            current_map_id: 0,
            save_interval: 0.0,
            show_only_active: true,
            currently_used_files: Default::default(),
            parents: Default::default(),
            loaded_elements: Default::default(),
            on_screen: Default::default(),
        })
    }

    pub fn set_currently_used_files(&mut self, currently_used_files: BTreeMap<String, bool>) {
        self.currently_used_files = currently_used_files;
    }

    pub fn category_set(&mut self, uuid: Uuid, status: bool) {
        for pack in self.packs.values_mut() {
            if pack.category_set(uuid, status) {
                break;
            }
        }
    }

    pub fn category_set_all(&mut self, status: bool) {
        for pack in self.packs.values_mut() {
            pack.category_set_all(status);
        }
    }

    pub fn register(&mut self, element: Uuid, parent: Uuid) {
        self.parents.insert(element, parent);
    }
    pub fn get_parent(&self, element: &Uuid) -> Option<&Uuid> {
        self.parents.get(element)
    }
    pub fn get_parents<'a, I>(&self, input: I) -> HashSet<Uuid>
    where I: Iterator<Item=&'a Uuid>
    {
        let iter = input.into_iter();
        let mut result: HashSet<Uuid> = HashSet::new();
        let mut current_generation: Vec<Uuid> = Vec::new();
        for elt in iter {
            current_generation.push(*elt)
        }
        //info!("starts with {}", current_generation.len());
        loop {
            if current_generation.is_empty() {
                //info!("ends with {}", result.len());
                return result;
            }
            let mut next_gen: Vec<Uuid> = Vec::new();
            for elt in current_generation.iter() {
                if let Some(p) = self.get_parent(elt) {
                    if result.contains(p) {
                        //avoid duplicate, redundancy or loop
                        continue;
                    }
                    next_gen.push(p.clone());
                }
            }
            let to_insert = std::mem::replace(&mut current_generation, next_gen);
            result.extend(to_insert);
        }
        unreachable!("The loop should always return");
    }

    pub fn get_active_elements_parents(&mut self, categories_and_elements_to_be_loaded: HashSet<Uuid>) {
        trace!("There are {} active elements", categories_and_elements_to_be_loaded.len());

        //first merge the parents to iterate overit
        let mut parents: HashMap<Uuid, Uuid> = Default::default();
        for pack in self.packs.values_mut() {
            parents.extend(pack.entities_parents.clone());
        }
        self.parents = parents;
        //then climb up the tree of parent's categories
        self.loaded_elements = self.get_parents(categories_and_elements_to_be_loaded.iter());
    }

    pub fn tick(
        &mut self,
        b2u_sender: &std::sync::mpsc::Sender<BackToUIMessage>,
        loop_index: u128,
        link: Option<&MumbleLink>,
        choice_of_category_changed: bool,
    ) {
        let mut currently_used_files: BTreeMap<String, bool> = Default::default();
        let mut categories_and_elements_to_be_loaded: HashSet<Uuid> = Default::default();
        
        match link {
            Some(link) => {
                //FIXME: how to save/load the active files ?
                //TODO: find an efficient way to propagate the file deactivation
                let mut have_used_files_list_changed = false;
                let map_changed = self.current_map_id != link.map_id;
                self.current_map_id = link.map_id;
                for pack in self.packs.values_mut() {
                    if let Some(current_map) = pack.maps.get(&link.map_id) {
                        for marker in current_map.markers.values() {
                            if let Some(is_active) = pack.source_files.get(&marker.source_file_name) {
                                currently_used_files.insert(
                                    marker.source_file_name.clone(), 
                                    *self.currently_used_files.get(&marker.source_file_name).unwrap_or_else(|| {have_used_files_list_changed = true; is_active})
                                );
                            }
                        }
                        for trail in current_map.trails.values() {
                            if let Some(is_active) = pack.source_files.get(&trail.source_file_name) {
                                currently_used_files.insert(
                                    trail.source_file_name.clone(), 
                                    *self.currently_used_files.get(&trail.source_file_name).unwrap_or_else(|| {have_used_files_list_changed = true; is_active})
                                );
                            }
                        }
                    }
                }
                let mut tasks = &self.tasks;
                for (uuid, pack) in self.packs.iter_mut() {
                    let span_guard = info_span!("Updating package status").entered();
                    tasks.save_data(pack, pack.is_dirty());
                    pack.tick(
                        &b2u_sender,
                        loop_index,
                        link,
                        &currently_used_files,
                        have_used_files_list_changed || choice_of_category_changed,
                        map_changed,
                        &tasks, 
                        &mut categories_and_elements_to_be_loaded,
                    );
                    std::mem::drop(span_guard);
                }
                if map_changed {
                    self.get_active_elements_parents(categories_and_elements_to_be_loaded);
                    b2u_sender.send(BackToUIMessage::ActiveElements(self.loaded_elements.clone()));
                }
                if map_changed || have_used_files_list_changed || choice_of_category_changed {
                    //there is no point in sending a new list if nothing changed
                    b2u_sender.send(BackToUIMessage::CurrentlyUsedFiles(currently_used_files.clone()));
                    self.currently_used_files = currently_used_files;
                    b2u_sender.send(BackToUIMessage::TextureSwapChain);
                }
            },
            None => {},
        };
        //TODO: state_sender.send(BackToUIMessage::ActiveElements(active_elements));
        
        
        //those are the elements displayed, not the categories, one would need to keep the link between the two
        /*if is_one_package_reloaded {
            for pack in self.packs.values() {
                next_loaded.extend(pack.active_elements());
            }
            info!("Loaded {} elements", next_loaded.len());
            self.loaded_elements = self.update_active_elements(next_loaded);
        }*/
        //self.on_screen = self.update_active_elements(next_on_screen);
    }

    pub fn save(&mut self, mut data_pack: LoadedPackData) {
        self.tasks.save_data(&mut data_pack, true);
        self.packs.insert(data_pack.uuid, data_pack);
    }

}


impl PackageUIManager {
    pub fn new(packs: BTreeMap<Uuid, LoadedPackTexture>) -> Self {
        Self {
            packs,
            tasks: PackTasks::new(),
            default_marker_texture: None,
            default_trail_texture: None,
            import_status: Default::default(),

            all_files_tribool: Tribool::True,
            all_files_toggle: false,
            show_only_active: true,
            currently_used_files: Default::default()// UI copy to (de-)activate files
        }
    }

    pub fn late_init(
        &mut self,
        etx: &egui::Context,
    ) {
        if self.default_marker_texture.is_none() {
            let img = image::load_from_memory(include_bytes!("../pack/marker.png")).unwrap();
            let size = [img.width() as _, img.height() as _];
            self.default_marker_texture = Some(etx.load_texture(
                "default marker",
                ColorImage::from_rgba_unmultiplied(size, img.into_rgba8().as_bytes()),
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Linear,
                    wrap_mode: egui::TextureWrapMode::ClampToEdge,
                },
            ));
        }
        if self.default_trail_texture.is_none() {
            let img = image::load_from_memory(include_bytes!("../pack/trail_rainbow.png")).unwrap();
            let size = [img.width() as _, img.height() as _];
            self.default_trail_texture = Some(etx.load_texture(
                "default trail",
                ColorImage::from_rgba_unmultiplied(size, img.into_rgba8().as_bytes()),
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Linear,
                    wrap_mode: egui::TextureWrapMode::ClampToEdge,
                },
            ));
        }
    }

    pub fn delete_packs(&mut self, to_delete: Vec<Uuid>) {
        for uuid in to_delete {
            self.packs.remove(&uuid);
        }
    }
    pub fn set_currently_used_files(&mut self, currently_used_files: BTreeMap<String, bool>) {
        self.currently_used_files = currently_used_files;
    }

    pub fn update_active_categories(&mut self, active_elements: &HashSet<Uuid>) {
        trace!("There are {} active elements", active_elements.len());
        for pack in self.packs.values_mut() {
            pack.update_active_categories(active_elements);
        }
    }

    pub fn update_pack_active_categories(&mut self, pack_uuid: Uuid, active_elements: &HashSet<Uuid>) {
        trace!("There are {} active elements", active_elements.len());
        for (uuid, pack) in self.packs.iter_mut() {
            if uuid == &pack_uuid {
                pack.update_active_categories(active_elements);
                break;
            }
        }
    }
    pub fn swap(&mut self) {
        for pack in self.packs.values_mut() {
            pack.swap();
        }
    }

    pub fn load_marker_texture(
        &mut self, 
        egui_context: &egui::Context, 
        pack_uuid: Uuid, 
        tex_path: RelativePath, 
        marker_uuid: Uuid, 
        position: Vec3,
        common_attributes: CommonAttributes,
    ) {
        self.packs
            .get_mut(&pack_uuid)
            .map( |pack| {
                pack.load_marker_texture(
                    egui_context, 
                    self.default_marker_texture.as_ref().unwrap(),
                    &tex_path, 
                    marker_uuid,
                    position,
                    common_attributes,
                );
            });
    }
    pub fn load_trail_texture(
        &mut self, 
        egui_context: &egui::Context, 
        pack_uuid: Uuid, 
        tex_path: RelativePath, 
        trail_uuid: Uuid, 
        common_attributes: CommonAttributes,
    ) {
        self.packs
            .get_mut(&pack_uuid)
            .map( |pack| {
                pack.load_trail_texture(
                    egui_context, 
                    &self.default_trail_texture.as_ref().unwrap(),
                    &tex_path, 
                    trail_uuid,
                    common_attributes,
                );
            });
    }

    fn pack_importer(import_status: Arc<Mutex<ImportStatus>>) {
        //called when a new pack is imported
        rayon::spawn(move || {
            *import_status.lock().unwrap() = ImportStatus::WaitingForFileChooser;

            if let Some(file_path) = rfd::FileDialog::new()
                .add_filter("taco", &["zip", "taco"])
                .pick_file()
            {
                *import_status.lock().unwrap() = ImportStatus::LoadingPack(file_path.clone());

                let result = import_pack_from_zip_file_path(file_path);
                match result {
                    Ok((name, pack)) => {
                        *import_status.lock().unwrap() = ImportStatus::PackDone(name, pack, false);
                    }
                    Err(e) => {
                        *import_status.lock().unwrap() = ImportStatus::PackError(e);
                    }
                }
            } else {
                *import_status.lock().unwrap() =
                    ImportStatus::PackError(miette::miette!("file chooser was cancelled"));
            }
        });
    }

    fn category_set_all(&mut self, status: bool) {
        for pack in self.packs.values_mut() {
            pack.category_set_all(status);
        }
    }

    pub fn tick(
        &mut self,
        u2u_sender: &std::sync::mpsc::Sender<UIToUIMessage>,
        timestamp: f64,
        link: &MumbleLink,
        z_near: f32,
    ) {
        let mut tasks = &self.tasks;
        for (uuid, pack) in self.packs.iter_mut() {
            let span_guard = info_span!("Updating package status").entered();
            tasks.save_texture(pack, pack.is_dirty());
            pack.tick(
                &u2u_sender,
                timestamp,
                link,
                z_near,
                &tasks
            );
            std::mem::drop(span_guard);
        }
        u2u_sender.send(UIToUIMessage::RenderSwapChain);
        //u2u_sender.send(UIToUIMessage::Present);
    }

    pub fn menu_ui(
        &mut self, 
        u2b_sender: &std::sync::mpsc::Sender<UIToBackMessage>,
        u2u_sender: &std::sync::mpsc::Sender<UIToUIMessage>,
        ui: &mut egui::Ui
    ) {
        ui.menu_button("Markers", |ui| {
            if self.show_only_active {
                if ui.button("Show everything").clicked() {
                    self.show_only_active = false;
                }
            } else {
                if ui.button("Show only active").clicked() {
                    self.show_only_active = true;
                }
            }
            if ui.button("Activate all elements").clicked() {
                self.category_set_all(true);
                u2b_sender.send(UIToBackMessage::CategorySetAll(true));
            }
            if ui.button("Deactivate all elements").clicked() {
                self.category_set_all(false);
                u2b_sender.send(UIToBackMessage::CategorySetAll(false));
            }

            for pack in self.packs.values_mut() {
                //pack.is_dirty = pack.is_dirty || force_activation || force_deactivation;
                //category_sub_menu is for display only, it's a bad idea to use it to manipulate status
                pack.category_sub_menu(u2b_sender, u2u_sender, ui, self.show_only_active);
            }
            
        });
        if self.tasks.is_running() {
            ui.spinner();
        }
    }

    fn gui_file_manager(
        &mut self, 
        event_sender: &std::sync::mpsc::Sender<UIToBackMessage>,
        etx: &egui::Context, 
        open: &mut bool, 
        link: Option<&MumbleLink>
    ) {
        let mut files_changed = false;
        Window::new("File Manager").open(open).show(etx, |ui| -> Result<()> {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("link grid")
                    .num_columns(4)
                    .striped(true)
                    .show(ui, |ui| {
                        if self.all_files_tribool.is_indeterminate(){
                            ui.add(egui::Checkbox::new(&mut self.all_files_toggle, "File").indeterminate(true));
                        } else {
                            ui.checkbox(&mut self.all_files_toggle, "File");
                        }
                        ui.label("Trails");
                        ui.label("Markers");
                        ui.end_row();
                        
                        for file in self.currently_used_files.iter_mut() {
                            let cb = ui.checkbox(file.1, file.0.clone());
                            if cb.changed() {
                                files_changed = true;
                            }
                            if ui.button("Edit").clicked() {
                                println!("click {}", file.0.clone());
                            }
                            ui.end_row();
                        }
                        ui.end_row();
                    })
            });
            Ok(())
        });
        if files_changed {
            event_sender.send(UIToBackMessage::ActiveFiles(self.currently_used_files.clone()));
        }
    }
    fn gui_package_loader(
        &mut self, 
        event_sender: &std::sync::mpsc::Sender<UIToBackMessage>,
        etx: &egui::Context, 
        open: &mut bool
    ) {
        Window::new("Package Loader").open(open).show(etx, |ui| -> Result<()> {
            CollapsingHeader::new("Loaded Packs").show(ui, |ui| {
                egui::Grid::new("packs").striped(true).show(ui, |ui| {
                    let mut to_delete = vec![];
                    for pack in self.packs.values() {
                        ui.label(pack.name.clone());
                        if ui.button("delete").clicked() {
                            to_delete.push(pack.uuid);
                        }
                    }
                    if !to_delete.is_empty() {
                        //TODO: send message to background thread, UIToBackMessage::DeletePack
                        event_sender.send(UIToBackMessage::DeletePacks(to_delete));
                    }
                });
            });

            if self.import_status.is_some() {
                if ui.button("clear").on_hover_text(
                    "This will cancel any pack import in progress. If import is already finished, then it wil simply clear the import status").clicked() {
                    self.import_status = None;
                }
            } else if ui.button("import pack").on_hover_text("select a taco/zip file to import the marker pack from").clicked() {
                //TODO: send message to background thread, UIToBackMessage::ImportPack
                let import_status = Arc::new(Mutex::default());
                self.import_status = Some(import_status.clone());
                Self::pack_importer(import_status);
            }
            if let Some(import_status) = self.import_status.as_ref() {
                if let Ok(mut status) = import_status.lock() {
                    match &mut *status {
                        ImportStatus::UnInitialized => {
                            ui.label("import not started yet");
                        }
                        ImportStatus::WaitingForFileChooser => {
                            ui.label(
                                "wailting for the file dialog. choose a taco/zip file to import",
                            );
                        }
                        ImportStatus::LoadingPack(p) => {
                            ui.label(format!("pack is being imported from {p:?}"));
                        }
                        ImportStatus::PackDone(name, pack, saved) => {

                            if !*saved {
                                ui.horizontal(|ui| {
                                    ui.label("choose a pack name: ");    
                                    ui.text_edit_singleline(name);
                                });
                                if ui.button("save").clicked() {
                                    event_sender.send(UIToBackMessage::SavePack(name.clone(), pack.clone()));
                                }
                            } else {
                                ui.colored_label(egui::Color32::GREEN, "pack is saved. press click `clear` button to remove this message");
                            }
                        }
                        ImportStatus::PackError(e) => {
                            ui.colored_label(
                                egui::Color32::RED,
                                format!("failed to import pack due to error: {e:#?}"),
                            );
                        }
                    }
                }
            }

            Ok(())
        });
    }
    pub fn gui(
        &mut self, 
        event_sender: &std::sync::mpsc::Sender<UIToBackMessage>,
        etx: &egui::Context, 
        is_marker_open: &mut bool, 
        is_file_open: &mut bool, 
        timestamp: f64,
        link: Option<&MumbleLink>
    ) {
        self.gui_package_loader(event_sender, etx, is_marker_open);
        self.gui_file_manager(event_sender, etx, is_file_open, link);
    }

    pub fn save(&mut self, mut texture_pack: LoadedPackTexture) {
        self.tasks.save_texture(&mut texture_pack, true);
        self.packs.insert(texture_pack.uuid, texture_pack);
    }
}


