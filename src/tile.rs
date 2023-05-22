use egui::{Color32, Frame, Id};
use macroquad::prelude::collections::storage;
use std::collections::HashMap;

use crate::{
    building::{Building, BuildingType, Resource},
    EditTool, SelectTool,
};

use std::time::Instant;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Savefile)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}



#[derive(Savefile)]

pub struct Tile {
    pub pos: Pos,
    pub land: [[Building; 8]; 8],
    pub buildings: HashMap<BuildingType, i32>,
    pub neighbors_buildings: HashMap<BuildingType, i32>,

    #[savefile_versions = "3.."]
    pub planned: HashMap<Pos, BuildingType>,
}

impl Tile {
    pub fn new(coord: Pos) -> Tile {
        let land: [[Building; 8]; 8] = Default::default();
        Tile {
            planned: HashMap::new(),
            pos: coord,
            land,
            buildings: HashMap::new(),
            neighbors_buildings: HashMap::new(),
        }
    }

    pub fn process_storage(&self, mut storage: i32, mut cash_storage: i32) -> (i32, i32) {
        for i in &self.buildings {
            for n in i.0.output() {
                match n.0 {
                    Resource::Storage => storage += n.1 * i.1,
                    Resource::CashStorage => cash_storage += n.1 * i.1,
                    _ => {}
                }
            }
        }
        return (storage, cash_storage);
    }

    pub fn processes_resources(
        &self,
        res: &mut HashMap<Resource, i32>,
        storage: i32,
        cash_storage: i32,
    ) {
        for i in &self.buildings {
            for n in i.0.output() {
                res.insert(
                    n.0,
                    (res.get(&n.0).unwrap_or(&0) + n.1 * i.1).min(match n.0 {
                        Resource::CashStorage => cash_storage + storage,
                        Resource::Storage => storage,
                        _ => storage,
                    }),
                );
            }
        }
    }

    fn update_count(&mut self, resources: &mut HashMap<Resource, i32>) {
        self.buildings.clear();
        for i in self.land.iter() {
            for j in i.iter() {
                
                self.buildings.insert(
                    j.building_type,
                    self.buildings.get(&j.building_type).unwrap_or(&0) + 1,
                );
            }
        }

        for x in 0..8 {
            for y in 0..8 {
                if !self.is_valid(Pos::new(x as i32, y as i32), &self.land[x][y]) {

                    self.planned.insert(Pos::new(x as i32, y as i32), self.land[x][y].building_type);
                    for c in &self.land[x][y].cost {
                        resources.insert(c.0, resources.get(&c.0).unwrap_or(&0) + c.1);
                    }
                    self.land[x][y] = Building::new(BuildingType::Ground);
                }
            }
        }

        
    }

    pub fn is_valid(&self, i: Pos, new_building: &Building) -> bool {
        if new_building.building_type == BuildingType::Ground {
            return true;
        }
        for requirement in &new_building.tile_adj {
            if let Some(_) = self.buildings.get(requirement) {
            } else {
                return false;
            }
        }
        let adj = i.get_adjacent()[0..4]
            .iter()
            .filter(|x| x.x < 8 && x.x >= 0 && x.y < 8 && x.y >= 0)
            .map(|x| self.land[x.x as usize][x.y as usize].building_type)
            .collect::<Vec<BuildingType>>();

        if !new_building.required_adj.iter().all(|x| adj.contains(x)) {
            return false;
        }
        if !adj
            .iter()
            .all(|x| new_building.required_adj.contains(x) || new_building.optional_adj.contains(x))
        {
            return false;
        }
        return true;
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for i in &self.land {
            for j in i {
                string.push_str(&j.symbol);
            }
            string.push('\n');
        }
        return string.trim_end().to_owned();
    }

    pub fn render(
        &mut self,
        egui_ctx: &egui::Context,
        input_settings: &crate::InputSettings,
        offset: (f32, f32),
        resources: &mut HashMap<Resource, i32>,
        enabled: bool,
    ) -> bool {
        let mut changed = false;
        let mut set_buildings: Vec<(Pos, Building)> = vec![];

        egui::Area::new(Id::new(self.pos.to_string()))
            .fixed_pos(egui::Pos2::new(
                offset.0 + self.pos.x as f32 * 202.0,
                offset.1 + self.pos.y as f32 * 202.0,
            ))
            .order(egui::Order::Background)
            .show(egui_ctx, |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 0.0);

                for i in 0..8 {
                    ui.horizontal(|ui| {
                        for j in 0..8 {
                            let text = &self.land[i][j].symbol.to_owned();

                            let square = egui::Button::new(&self.land[i][j].symbol)
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(200, 235, 200),
                                ))
                                .fill(
                                    
                                    match self.is_valid(Pos::new(i as i32, j as i32), &self.land[i][j]) {
                                        true => egui::Color32::from_rgb(215, 235, 210),
                                        false => egui::Color32::from_rgb(235, 215, 210),
                                    })
                                    
                                    // egui::Color32::from_rgb(215, 235, 210))
                                .small()
                                .min_size(egui::Vec2::new(25.0, 25.0))
                                .sense(egui::Sense::click_and_drag());
                            let response = ui.add_enabled(
                                !(enabled
                                    && ui.rect_contains_pointer(
                                        egui::Rect::from_two_pos(
                                            egui::Pos2::new(
                                                offset.0 + self.pos.x as f32 * 202.0,
                                                offset.1 + self.pos.y as f32 * 202.0,
                                            ),
                                            egui::Pos2::new(
                                                offset.0 + (self.pos.x + 1) as f32 * 202.0,
                                                offset.1 + (self.pos.y + 1) as f32 * 202.0,
                                            ),
                                        )
                                        .intersect(ui.cursor()),
                                    )),
                                square,
                            );

                            let text = match &input_settings.edit_tool {
                                EditTool::Build(b) => {
                                    if self.is_valid(
                                        Pos {
                                            x: i as i32,
                                            y: j as i32,
                                        },
                                        b,
                                    ) && self.land[i][j].building_type == BuildingType::Ground
                                    {
                                        egui::RichText::new(&b.symbol).to_owned()
                                    } else {
                                        egui::RichText::new("").to_owned()
                                    }
                                }
                                .italics(),
                                EditTool::Remove => match text.len() {
                                    2 => egui::RichText::new("").to_owned(),
                                    _ => egui::RichText::new(egui_phosphor::X.to_owned())
                                        .color(egui::Color32::from_rgb(255, 0, 0))
                                        .to_owned(),
                                },
                            };
                            if response.hovered() || ui.input(|r| r.key_down(egui::Key::Q)) {
                                ui.put(response.rect, egui::Label::new(text));
                            }
                            if let Some(b) = self.planned.get(&Pos {
                                x: i as i32,
                                y: j as i32,
                            }) {
                                ui.put(
                                    response.rect,
                                    egui::Label::new(
                                        egui::RichText::new(&b.symbol()).weak().to_owned(),
                                    ),
                                );

                                set_buildings.push((
                                    Pos {
                                        x: i as i32,
                                        y: j as i32,
                                    },
                                    Building::new(b.clone()),
                                ));
                            }

                            if response.clicked() {
                                if input_settings.select_tool == SelectTool::Plan {
                                    self.planned.insert(
                                        Pos {
                                            x: i as i32,
                                            y: j as i32,
                                        },
                                        match &input_settings.edit_tool {
                                            crate::EditTool::Build(b) => b.building_type.clone(),
                                            crate::EditTool::Remove => BuildingType::Ground,
                                        },
                                    );
                                }
                                set_buildings.push((
                                    Pos {
                                        x: i as i32,
                                        y: j as i32,
                                    },
                                    match &input_settings.edit_tool {
                                        crate::EditTool::Build(b) => b.clone(),
                                        crate::EditTool::Remove => {
                                            Building::new(BuildingType::Ground)
                                        }
                                    },
                                ));
                            }
                        }
                    });
                }
                // });
            });

        for (i, new_building) in set_buildings {
            let mut break_building = false;

            let ground = Building::new(BuildingType::Ground);

            let current = &self.land[i.x as usize][i.y as usize];
            if current.building_type == new_building.building_type {
                self.planned.remove(&i);
                continue;
            }

            if new_building.building_type == BuildingType::Ground {
                for i in Building::new(current.building_type).cost {
                    let storage = resources.get(&Resource::Storage).unwrap_or(&100);
                    resources.insert(i.0, (resources.get(&i.0).unwrap_or(&0) + i.1).min(*storage));
                }

                self.land[i.x as usize][i.y as usize] = ground;
                self.update_count(resources);
                changed = true;
                continue;
            }

            if !self.is_valid(i, &new_building) {
                continue;
            }

            for i in new_building.cost.iter() {
                if let Some(x) = resources.get(&i.0) {
                    if *x < i.1 {
                        break_building = true;
                    }
                } else {
                    break_building = true;
                }
            }

            if break_building {
                continue;
            }


            self.planned.remove(&i);

            for i in new_building.cost.iter() {
                if let Some(x) = resources.get_mut(&i.0) {
                    *x -= i.1;
                }
            }

            self.land[i.x as usize][i.y as usize] = new_building.clone();
            changed = true;
        }
        if changed {
            self.update_count(resources);
        }

        return changed;
    }
}
