
#![cfg_attr(
  all(
    target_os = "windows",
    not(debug_assertions),
  ),
  windows_subsystem = "windows"
)]

use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    time::Instant,
};

const GLOBAL_VERSION: u32 = 6;

use egui::{
    self, epaint::Shadow, Align2, Button, Frame, Id, LayerId, Pos2, Sense, TextStyle, Vec2, Order, Color32,
};
use egui_macroquad;
use egui_phosphor;
use macroquad::prelude::*;

extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;

mod building;
mod tile;

use building::*;
use tile::*;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

impl Pos {
    fn to_string(&self) -> String {
        format!("({},{})", self.x, self.y)
    }
    pub fn new<T: Into<i32>>(x:T, y:T) -> Pos {
        Pos { x: x.into(), y: y.into() }
    }
    fn added(&self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn get_adjacent(&self) -> [Pos; 5] {
        [
            self.added(Pos::new(-1, 0)),
            self.added(Pos::new(1, 0)),
            self.added(Pos::new(0, -1)),
            self.added(Pos::new(0, 1)),
            *self,
        ]
    }

    fn cost(&self) -> i32 {
        (self.x.abs().max(self.y.abs()) + 1).pow(3) * 100
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, EnumIter)]
pub enum SelectTool {
    Add,
    Plan,
}

impl SelectTool {
    fn icon(&self) -> String {
        (match self {
            SelectTool::Add => egui_phosphor::CURSOR_CLICK,
            SelectTool::Plan => egui_phosphor::CIRCLE_DASHED,
        })
        .to_owned()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum EditTool {
    Build(Building),
    Remove,
}

impl EditTool {
    fn icon(&self) -> String {
        (match self {
            EditTool::Build(b) => b.symbol.clone(),
            EditTool::Remove => egui_phosphor::ERASER.to_owned(),
        })
        .to_owned()
    }
}

pub struct InputSettings {
    pub select_tool: SelectTool,
    pub edit_tool: EditTool,
}

impl Default for InputSettings {
    fn default() -> Self {
        InputSettings {
            select_tool: SelectTool::Add,
            edit_tool: EditTool::Build(Building::new(&BuildingType::House)),
        }
    }
}



fn no_rect() -> egui::Rect {
    egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(0.0, 0.0))
}
fn default_float_tuple() -> (f32, f32) {
    (100.0, 100.0)
}

#[derive(Savefile)]
pub struct Data {
    #[savefile_default_val = "Old World"]
    #[savefile_versions = "2.."]
    pub name: String,



    pub tiles: HashMap<Pos, Tile>,

    #[savefile_introspect_ignore]
    #[savefile_ignore]
    pub input_settings: InputSettings,

    #[savefile_default_fn = "default_float_tuple"]
    #[savefile_introspect_ignore]
    #[savefile_ignore]
    pub screen_offset: (f32, f32),
    pub new_pos: Vec<Pos>,
    pub resources: HashMap<Resource, i32>,

    pub stage: [Stage; 5],
    ui_scale: f32,
    game_scale: f32,



    #[savefile_introspect_ignore]
    #[savefile_default_fn = "no_rect"]
    #[savefile_ignore]
    switch_tool_rect: egui::Rect,

    #[savefile_introspect_ignore]
    #[savefile_default_fn = "no_rect"]
    #[savefile_ignore]
    select_building_rect: egui::Rect,

    #[savefile_default_val = "false"]
    #[savefile_ignore]
    popup: bool,

    #[savefile_default_val = "false"]
    #[savefile_ignore]
    popup_hover: bool,

    #[savefile_default_val = "false"]
    #[savefile_versions = "4.."]
    quick_menu: bool,

}

impl Data {
    fn new(name: String) -> Data {
        let mut d = Data {
            name,
            quick_menu: false,
            switch_tool_rect: egui::Rect::from_min_size(
                egui::Pos2::new(0.0, 0.0),
                egui::Vec2::new(0.0, 0.0),
            ),
            select_building_rect: egui::Rect::from_min_size(
                egui::Pos2::new(0.0, 0.0),
                egui::Vec2::new(0.0, 0.0),
            ),
            ui_scale: 1.0,
            game_scale: 1.0,
            new_pos: vec![Pos::new(0, 0)],
            tiles: HashMap::new(),
            input_settings: InputSettings {
                select_tool: SelectTool::Add,
                edit_tool: EditTool::Build(Building::new(&BuildingType::House)),
            },
            screen_offset: (100.0, 100.0),
            resources: HashMap::new(),
            stage: [
                Stage::new(1),
                Stage::new(2),
                Stage::new(3),
                Stage::new(4),
                Stage::new(5),
            ],
            popup: false,
            popup_hover: false,
        };
        for r in Resource::iter() {
            d.resources.insert(r, 0);
        }
        d.resources.insert(Resource::Seed, 10);
        d.resources.insert(Resource::Food, 10);
        d.resources.insert(Resource::Wood, 10);
        d.resources.insert(Resource::Storage, 100);
        d.tiles
            .insert(Pos { x: 0, y: 0 }, Tile::new(Pos { x: 0, y: 0 }));
        d.update_new_pos();

        d
    }

    fn update_new_pos(&mut self) {
        let mut tiles: HashSet<Pos> = HashSet::new();
        let mut new_tiles: HashSet<Pos> = HashSet::new();
        for i in &self.tiles {
            tiles.insert(*i.0);
            i.0.get_adjacent().iter().for_each(|x| {
                new_tiles.insert(*x);
            });
        }
        self.new_pos = new_tiles.difference(&tiles).cloned().collect();
    }

    fn add_buildings(&mut self, pos: [Pos; 5]) {
        let mut new_hash = HashMap::new();

        for p in pos {
            if let Some(b) = self.tiles.get(&p) {
                for i in &b.buildings {
                    new_hash.insert(i.0.clone(), new_hash.get(&i.0).unwrap_or(&0) + i.1);
                }
            }
        }
        
        if let Some(mut b) = self.tiles.get_mut(&pos[4]) {
            b.neighbors_buildings = new_hash;
        }
    }

    fn render(&mut self, egui_ctx: &egui::Context) {
        let mut update_adjacent: Option<Pos> = None;

        

        for i in &mut self.tiles {
            if egui_ctx.screen_rect().contains(egui::Pos2::new(
                self.screen_offset.0 + (i.0.x as f32) * 202.0,
                self.screen_offset.1 + (i.0.y as f32) * 202.0,
            )) || egui_ctx.screen_rect().contains(egui::Pos2::new(
                self.screen_offset.0 + (i.0.x as f32) * 202.0 + 202.0,
                self.screen_offset.1 + (i.0.y as f32) * 202.0 + 202.0,
            )) || egui_ctx.screen_rect().contains(egui::Pos2::new(
                self.screen_offset.0 + (i.0.x as f32) * 202.0,
                self.screen_offset.1 + (i.0.y as f32) * 202.0 + 202.0,
            )) || egui_ctx.screen_rect().contains(egui::Pos2::new(
                self.screen_offset.0 + (i.0.x as f32) * 202.0 + 202.0,
                self.screen_offset.1 + (i.0.y as f32) * 202.0,
            )) {
                if i.1.render(
                    egui_ctx,
                    &self.input_settings,
                    self.screen_offset,
                    &mut self.resources,
                    self.popup_hover,
                ) {
                    update_adjacent = Some(*i.0);
                }
            }
        }
        let mut add: Vec<Pos> = vec![];
        for i in &self.new_pos {
            if &i.cost() <= self.resources.get(&Resource::Tax).unwrap_or(&0)
                
            {
                egui::Window::new(i.to_string())
                    .title_bar(false)
                    .resizable(false)
                    .collapsible(false)
                    .frame(Frame::none())
                    .fixed_rect(egui::Rect::from_two_pos(
                        Pos2::new(
                            self.screen_offset.0
                                + (i.x as f32) * 202.0
                                + (match i.x < 0 {
                                    true => 100.0,
                                    false => 0.0,
                                }),
                                
                            self.screen_offset.1
                                + (i.y as f32) * 202.0
                                + (match i.y < 0 {
                                    true => 180.0,
                                    false => 0.0,
                                }),
                        ),
                        Pos2::new(
                            self.screen_offset.0 + ((i.x + 1) as f32) * 202.0,
                            self.screen_offset.1 + ((i.y + 1) as f32) * 240.0,
                        ),
                    ))
                    .show(egui_ctx, |ui| {
                        ui.add(egui::widgets::Button::new(format!("${}", i.cost())))
                            .clicked()
                            .then(|| {
                                *self.resources.get_mut(&Resource::Tax).unwrap_or(&mut 0) -=
                                    i.cost();
                                add.push(*i);
                            });
                    });
            }
        }
        for i in add {
            self.tiles.insert(i, Tile::new(i));
            self.update_new_pos();
        }

        if let Some(pos) = update_adjacent {
                self.add_buildings(pos.get_adjacent());
        }
    }
}

#[macroquad::main("egui with macroquad")]
async fn main() {

    let mut stats = false;

    let mut per_sec:HashMap<Resource, i32> = HashMap::new();
    let mut data = Data::new("".to_owned());
    let mut menu = true;
    match savefile::load_file::<Data, &str>("game_instance.bin", GLOBAL_VERSION) {
        Ok(d) => {
            // create new folder called game files and then move the file "game_instance.bin" into it
            println!("{:?}",std::fs::create_dir("saves"));
            savefile::save_file("saves/game_instanceOld World.bin", GLOBAL_VERSION, &d).unwrap();
        }
        Err(e) => {
            println!("{e}")
        }
    };
    let mut og_ppp = 0.0;
    println!("Test 3");

    egui_macroquad::ui(|egui_ctx| {
        og_ppp = egui_ctx.pixels_per_point();
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts);
        egui_ctx.set_fonts(fonts);
        egui_ctx.set_visuals(egui::Visuals::light());
        let mut style = (*egui_ctx.style()).clone();
        style.override_text_style = Some(TextStyle::Monospace);
    });

    let mut mouse_down = None;
    let mut offset_start = (0.0, 0.0);
    let mut start_in_area = false;

    let mut process = Instant::now();
    loop {
        clear_background(WHITE);
        let mut hover_text: Option<String> = None;
        if menu {
            egui_macroquad::ui(|egui_ctx| {

                egui::SidePanel::right("Right side").show(egui_ctx, |ui| {
                    if let Ok(files) = std::fs::read_dir("saves/") {
                        ui.heading("Saved Games");
                        
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                        for f in files {
                            let f = match f {Ok(f) => f, _ => {continue;}};
                            if f.file_type().unwrap().is_file() {
                                
                                ui.add(egui::widgets::Button::new(f.file_name().to_str().unwrap()))
                                    .clicked()
                                    .then(|| {
                                        match  savefile::load_file::<Data, &str>(
                                            &format!("saves/{}", f.file_name().to_str().unwrap()),GLOBAL_VERSION) {
                                            Ok(load) => {
                                                data = load;
                                            menu = false;
                                            }
                                            Err(e) => {
                                                println!("{:?}", e);
                                            }
                                        }
                                        
                                    });
                                }
                            }
                        });
                    }else {
                        ui.label("Error loading files");
                    }
                });

                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.heading("Welcome to the game!");
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::widgets::TextEdit::singleline(&mut data.name)
                            .hint_text("New Game Name")
                            .cursor_at_end(true)
                            
                        );
                            
                        
                        if data.name != "".to_owned() && ui.add(egui::widgets::Button::new("New Game"))
                            .clicked()
                            {
                                menu = false;
                            }
                        
                        
                    });
                });
            });
            egui_macroquad::draw();

            next_frame().await;
            continue;
        }

        if process.elapsed().as_secs() >= 3 {
            let filename = format!("saves/game_instance_{}.bin", data.name);
            process = Instant::now();
            let mut storage = 100;
            let mut cash_storage = 0;
            for i in &mut data.tiles {
                (storage, cash_storage) = i.1.process_storage(storage, cash_storage);
            }
            per_sec.clear();
            for i in &mut data.tiles {
                i.1.processes_resources(&mut data.resources, storage, cash_storage, &mut per_sec) 
                
            }

            for i in &mut data.stage {
                if !i.enabled {
                    if i.unlock_at
                        .iter()
                        .all(|x| data.resources.get(&x.0).unwrap_or(&0) >= &x.1)
                    {
                        i.enabled = true;
                    }
                }
            }
            match savefile::save_file(&filename, GLOBAL_VERSION, &data) {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        egui_macroquad::ui(|egui_ctx| {
            let mut unlock = -1;

            data.render(egui_ctx);

            let window_id = Id::new("popup selector");
            let mut popup_layer_id = egui::LayerId::new(egui::Order::Foreground, window_id);
            if data.popup {
                match egui::Window::new(format!("Build: {}", &data.input_settings.edit_tool.icon()))
                    .collapsible(true)
                    .id(window_id)
                    .open(&mut data.popup)
                    .scroll2([false, true])
                    .show(egui_ctx, |ui| {
                        // egui_ctx.move_to_top(ui.layer_id());

                        ui
                            .vertical(|ui| {
                                ui.checkbox(&mut data.quick_menu, "Quick Menu");
                                if data.quick_menu {
                                    ui.small_button("Remove").clicked().then(|| {
                                        data.input_settings.edit_tool = EditTool::Remove;
                                    });
                                    for s in data.stage.iter_mut() {
                                        ui.separator();
                                        if s.enabled {
                                            ui.horizontal(|ui| {
                                                for b in &s.buildings {
                                                    if ui.small_button(format!("{}", b.symbol())).clicked() {
                                                        data.input_settings.edit_tool = EditTool::Build(Building::new(b));
                                                    }
                                                }
                                            });
                                        } else {
                                            ui.small_button(format!("{} unlock early", egui_phosphor::LOCK)).clicked().then(|| {
                                                s.enabled = true;
                                            });
                                        }
                                    }
                                }else{
                            
                                    if ui.small_button("Delete").clicked() {
                                        data.input_settings.edit_tool = EditTool::Remove;
                                    }
                                popup_layer_id = ui.layer_id();
                                for i in &data.stage {
                                    if i.enabled {
                                        ui.heading(format!(
                                            "{} Stage {}",
                                            egui_phosphor::LOCK_OPEN,
                                            i.num
                                        ));
                                        ui.group(|ui| {
                                            for j in &i.buildings {
                                                let building = Building::new(j);
                                                if ui
                                                    .small_button(format!(
                                                        "{} {}",
                                                        j.symbol(),
                                                        j.name()
                                                    ))
                                                    .clicked()
                                                {
                                                    data.input_settings.edit_tool =
                                                        EditTool::Build(Building::new(j));
                                                }
                                                egui::Grid::new(j.name() + &i.title).show(
                                                    ui,
                                                    |ui| {
                                                        egui::CollapsingHeader::new(
                                                            "Cost ".to_owned()
                                                                + &building
                                                                    .cost
                                                                    .iter()
                                                                    .map(|x| {
                                                                        format!(
                                                                            "{} {},",
                                                                            x.0.symbol(),
                                                                            x.1
                                                                        )
                                                                    })
                                                                    .collect::<String>(),
                                                        )
                                                        .id_source(j.name() + "cost" + &i.title)
                                                        .show(ui, |ui| {
                                                            for k in building.cost.iter() {
                                                                ui.label(format!(
                                                                    "{} {}",
                                                                    k.0.name(),
                                                                    k.1
                                                                ));
                                                            }
                                                        });
                                                        ui.end_row();
                                                        if j.output().len() > 0 {
                                                            egui::CollapsingHeader::new(
                                                                "Output ".to_owned()
                                                                    + &j.output()
                                                                        .iter()
                                                                        .map(|x| {
                                                                            format!(
                                                                                "{} {},",
                                                                                x.0.symbol(),
                                                                                x.1
                                                                            )
                                                                        })
                                                                        .collect::<String>(),
                                                            )
                                                            .id_source(
                                                                j.name() + "output" + &i.title,
                                                            )
                                                            .show(ui, |ui| {
                                                                for k in j.output().iter() {
                                                                    ui.label(format!(
                                                                        "{} {}",
                                                                        k.0.name(),
                                                                        k.1
                                                                    ));
                                                                }
                                                            });
                                                            ui.end_row();
                                                        }
                                                        if building.required_adj.len() > 0 {
                                                            egui::CollapsingHeader::new(
                                                                "must be next to: ".to_owned()
                                                                    + &building
                                                                        .required_adj
                                                                        .iter()
                                                                        .map(|x| {
                                                                            x.symbol().replace(
                                                                                "  ",
                                                                                "[empty space]",
                                                                            )
                                                                        })
                                                                        .collect::<String>(),
                                                            )
                                                            .id_source(
                                                                j.name()
                                                                    + "must be next to: "
                                                                    + &i.title,
                                                            )
                                                            .show(ui, |ui| {
                                                                for k in building.required_adj {
                                                                    ui.label(k.name());
                                                                }
                                                            });
                                                            ui.end_row();
                                                        }

                                                        egui::CollapsingHeader::new(
                                                            "can be next to: ".to_owned()
                                                                + &building
                                                                    .optional_adj
                                                                    .iter()
                                                                    .map(|x| {
                                                                        x.symbol().replace(
                                                                            "  ",
                                                                            "[empty space]",
                                                                        )
                                                                    })
                                                                    .collect::<String>(),
                                                        )
                                                        .id_source(
                                                            j.name()
                                                                + "can be next to: "
                                                                + &i.title,
                                                        )
                                                        .show(ui, |ui| {
                                                            for k in building.optional_adj {
                                                                ui.label(k.name());
                                                            }
                                                        });
                                                        ui.end_row();
                                                        if building.tile_adj.len() > 0 {
                                                            egui::CollapsingHeader::new(
                                                                "on the same tile as: ".to_owned()
                                                                    + &building
                                                                        .tile_adj
                                                                        .iter()
                                                                        .map(|x| {
                                                                            x.symbol().replace(
                                                                                "  ",
                                                                                "[empty space]",
                                                                            )
                                                                        })
                                                                        .collect::<String>(),
                                                            )
                                                            .id_source(
                                                                j.name()
                                                                    + "on the same tile as: "
                                                                    + &i.title,
                                                            )
                                                            .show(ui, |ui| {
                                                                for k in building.tile_adj {
                                                                    ui.label(k.name());
                                                                }
                                                            });
                                                            ui.end_row();
                                                        }
                                                    },
                                                );
                                                ui.add_sized(
                                                    [ui.available_width(), 0.0],
                                                    egui::Label::new(""),
                                                );
                                            }
                                        });
                                    } else {
                                        ui.heading(format!(
                                            "{} Stage {}",
                                            egui_phosphor::LOCK,
                                            i.num
                                        ));
                                        ui.group(|ui| {
                                            ui.label("Unlock at");
                                            for resource in &i.unlock_at {
                                                egui::Grid::new(&i.title).show(ui, |ui| {
                                                    ui.label(format!(
                                                        "{} {} {}",
                                                        resource.0.symbol(),
                                                        resource.0.name(),
                                                        resource.1
                                                    ));
                                                });
                                            }
                                            if ui.small_button("Unlock Early").clicked() {
                                                unlock = i.num - 1;
                                            }
                                            ui.add_sized(
                                                [ui.available_width(), 0.0],
                                                egui::Label::new(""),
                                            );
                                        });
                                    }
                                }
                            }
                            });
                    }) {

                    Some(a) => {
                        if let Some(p) = egui_ctx.pointer_hover_pos() { 
                        data.popup_hover = a.response.rect.signed_distance_to_pos(p) < 0.0;
                    }
                    },
                    None => {},
};
            }

            // egui_ctx.move_to_top(popup_layer_id);

            if unlock != -1 && data.stage.len() > (unlock as usize) {
                data.stage[unlock as usize].enabled = true;
            }

            egui_ctx.input(|o| {
                if o.pointer.primary_clicked() && !data.popup_hover {
                    if data
                        .switch_tool_rect
                        .contains(o.pointer.hover_pos().unwrap())
                    {
                        
                        data.input_settings.select_tool = match &data.input_settings.select_tool {
                            SelectTool::Add => SelectTool::Plan,
                            SelectTool::Plan => SelectTool::Add,
                        };
                    }
                    if data
                        .select_building_rect
                        .contains(o.pointer.hover_pos().unwrap())
                    {
                        data.popup = !data.popup;
                    }
                }

                let is_none = mouse_down.is_none();
                mouse_down = o.pointer.press_origin();
                if is_none {
                    offset_start = data.screen_offset;
                    start_in_area = egui_ctx.is_pointer_over_area();
                }
                if !start_in_area {
                    if mouse_down.is_some() && !data.popup_hover {
                        if let Some(pos) = o.pointer.hover_pos() {
                            data.screen_offset = (
                                offset_start.0 + pos.x - mouse_down.unwrap().x,
                                offset_start.1 + pos.y - mouse_down.unwrap().y,
                            );
                        }
                    }
                }
            });
            egui::Area::new("data")
            .fixed_pos(egui::Pos2::new(5.0, 5.0))
            
            .order(Order::Foreground)
            .show(egui_ctx, |ui| {
                ui.group(|ui| {
                egui::Grid::new("grid")
                                        .show(ui, |ui| {

                    let show_full_data = ui.rect_contains_pointer(
                        egui::Rect::from_min_size(
                            egui::Pos2::new(0.0, 0.0),
                            egui::Vec2::new(100.0, 500.0),
                        ),
                    );
                        
                    let max_storage = data.resources.get(&Resource::Storage).unwrap_or(&0);
                    let max_cash = data.resources.get(&Resource::CashStorage).unwrap_or(&0);
                    for i in data.resources.iter() {
                        if i.1 == &0 {
                            continue;
                        }
                            if show_full_data {
                                ui.label(format!("{} {}", i.0.symbol(), i.0.name()));
                                ui.label(format!("{}", i.1));
                                ui.label(format!("{}/s", (*per_sec.get(i.0).unwrap_or(&0) as f32/0.3).round()/10.0));
                                ui.label(format!("{} %",( (*i.1 as f32)/(*max_storage as f32 + match i.0 {
                                    Resource::Tax => *max_cash as f32,
                                    _ => 0.0,
                                })*100.0).round()));

                                ui.end_row();
                            } else {
                                ui.label(format!("{} {}",i.0.symbol(), i.1));
                                ui.end_row();
                            }

                        }
                    });
                });
            });

            egui::Area::new("buttons")
            .anchor(Align2::LEFT_BOTTOM, [30.0,-30.0])
            .order(Order::Foreground)
            
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                                let tool = ui.heading(&data.input_settings.select_tool.icon());
                                data.switch_tool_rect = tool.rect;
                                ui.add_space(10.0);
                                data.select_building_rect =
                                ui.heading(&data.input_settings.edit_tool.icon()).rect;
                                    
            });
            });

        });
        egui_macroquad::draw();

        // egui_macroquad::ui(|egui_ctx| {

        //     egui::SidePanel::left("side_panel")
        //         .frame(Frame::none().inner_margin(10.0))
        //         .resizable(false)
        //         .show_separator_line(false)
        //         .show(egui_ctx, |ui| {

        //             egui::Window::new("Controls")
        //                 .anchor(Align2::LEFT_BOTTOM, egui::Vec2 { x: 30.0, y: 30.0 })
        //                 .frame(Frame::none())
        //                 .title_bar(false)
        //                 .default_pos(egui::Pos2::new(00.0, 30.0))
        //                 .constrain(true)
        //                 .movable(false)
        //                 .auto_sized()
        //                 .show(egui_ctx, |ui| {
        //                     ui.horizontal(|ui| {
        //                         let tool = ui.heading(&data.input_settings.select_tool.icon());
        //                         data.switch_tool_rect = tool.rect;
        //                         ui.add_space(10.0);
        //                         data.select_building_rect =
        //                             ui.heading(&data.input_settings.edit_tool.icon()).rect;
                                    
        //                     });

        //                     ui.add_space(30.0);
        //                 });
        //         });
        // });
        // egui_macroquad::draw();

        // check if in dev mode
        #[cfg(debug_assertions)]
        {
            draw_text("debug build", 10.0, 10.0, 15.0, BLACK);
            draw_text(&format!("fps: {}", get_fps()), 30.0, 30.0, 20.0, RED);
            
        }

        next_frame().await;
    }
}
