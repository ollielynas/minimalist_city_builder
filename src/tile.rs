use egui::{ Frame, Color32 };
use macroquad::prelude::collections::storage;
use std::collections::HashMap;

use crate::{ building::{ Building, BuildingType, Resource } };

use std::time::Instant;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}


pub struct Tile {
    pub pos: Pos,
    pub land: [[Building; 8]; 8],
    pub buildings: HashMap<BuildingType, i32>,
    pub neighbors_buildings: HashMap<BuildingType, i32>,
}

impl Tile {
    pub fn new(coord: Pos) -> Tile {
        let land: [[Building; 8]; 8] = Default::default();
        Tile {
            pos: coord,
            land,
            buildings: HashMap::new(),
            neighbors_buildings: HashMap::new(),
        }
    }

    pub fn process_storage(&self,  mut storage: i32,  mut cash_storage: i32) -> (i32,i32) {
        for i in &self.buildings {
            for n in i.0.output() {
                match n.0 {
                    Resource::Storage => storage += n.1 * i.1,
                    Resource::CashStorage => cash_storage += n.1 * i.1,
                    _ => {}
                }
            }
        }
        return (storage, cash_storage)
    }

    pub fn processes_resources(&self, res: &mut HashMap<Resource, i32>, storage: i32, cash_storage: i32) {
        
        
        for i in &self.buildings {
            for n in i.0.output() {
                res.insert(n.0, (res.get(&n.0).unwrap_or(&0) + n.1 * i.1).min(match n.0 {Resource::CashStorage => cash_storage+storage, Resource::Storage => storage, _ => storage}));
            }
        }
    }

    fn update_count(&mut self) {
        println!("update_count");
        self.buildings.clear();
        for i in &self.land {
            for j in i {
                self.buildings.insert(
                    j.building_type,
                    self.buildings.get(&j.building_type).unwrap_or(&0) + 1
                );
            }
        }
    }

    pub fn is_valid(&self, i:Pos, new_building:Building) -> bool {
        let mut break_building=false;
        for requirement in &new_building.tile_adj {
                if let Some(_) = self.neighbors_buildings.get(requirement) {
                } else {
                    break_building=true;
                }
            }
            let adj = i
                .get_adjacent()[0..4]
                .iter()
                .filter(|x| x.x < 8 && x.x >= 0 && x.y < 8 && x.y >= 0)
                .map(|x| self.land[x.x as usize][x.y as usize].building_type)
                .collect::<Vec<BuildingType>>();

            if !new_building.required_adj.iter().all(|x| adj.contains(x)) {
                break_building=true;
            }
            if
                !adj
                    .iter()
                    .all(
                        |x|
                            new_building.required_adj.contains(x) ||
                            new_building.optional_adj.contains(x)
                    )
            {
                break_building=true;
            }

            
        return !break_building
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
        resources: &mut HashMap<Resource, i32>
    ) -> bool {
        let mut changed = false;
        let mut set_buildings: Vec<Pos> = vec![];

        egui::Window
            ::new(self.pos.to_string())
            .title_bar(false)
            .resizable(false)
            .auto_sized()
            .collapsible(false)
            .constrain(false)
            .fixed_pos(egui::Pos2::new(offset.0 + self.pos.x as f32 *300.0, offset.1 + self.pos.y as f32*280.0))
            .frame(
                Frame::none()
                    .fill(egui::Color32::from_rgb(215, 235, 210))
                    .inner_margin(3.0)
                    .rounding(6.0)
            )

            .resizable(false)
            .collapsible(false)

            .show(egui_ctx, |ui| {
                let hover = egui_ctx.is_pointer_over_area();
                for i in 0..8 {
                    ui.horizontal(|ui| {
                        for j in 0..8 {
                            let square = egui::Button
                                ::new(&self.land[i][j].symbol)
                                .fill(Color32::TRANSPARENT)
                                // .frame(false)
                                .small()
                                .min_size(egui::Vec2::new(25.0, 25.0))
                                .sense(egui::Sense::click_and_drag());
                            let response = ui.add(square);

                            match input_settings.select_tool {
                                crate::SelectTool::Click => {
                                    if response.clicked() {
                                        set_buildings.push(Pos { x: i as i32, y: j as i32 });
                                    }
                                }
                                crate::SelectTool::Rect => {
                                    // if response.clicked() {
                                    //     self.land[i][j].building_type = input_settings.edit_tool.get_building();
                                    //     changed = true;
                                    //     set_buildings.push(Pos{x: i as i32, y: j as i32});
                                    // }
                                }
                            }
                        }
                    });
                }
                
            });

        for i in set_buildings {
            let mut break_building = false;

            
            let ground = Building::new(BuildingType::Ground);
            let new_building = match &input_settings.edit_tool {
                crate::EditTool::Build(b) => b,
                crate::EditTool::Remove => &ground,
            };
            let current = &self.land[i.x as usize][i.y as usize];
            if current.building_type == new_building.building_type {
                continue;
            }

            if new_building.building_type == BuildingType::Ground {

                for i in Building::new(current.building_type).cost {
                    let storage = resources.get(&Resource::Storage).unwrap_or(&100);
                    resources.insert(i.0, (resources.get(&i.0).unwrap_or(&0) + (i.1 as f32 * 0.75).round() as i32).min(*storage));
                }

                self.land[i.x as usize][i.y as usize] = ground;
                self.update_count();
                changed = true;
                continue;
            } 

            
            if self.is_valid(i, new_building.clone()) {
            }else {
                continue;
            }



            for i in new_building.cost.iter() {
                if let Some(x) = resources.get(&i.0) {
                    if *x < i.1 {
                        break_building=true;
                    }
                } else {
                    break_building=true;
                }
            }
            
            if break_building {
                continue;
            }

            for i in new_building.cost.iter() {
                if let Some(x) = resources.get_mut(&i.0) {
                    *x -= i.1;
                }
            }

            self.land[i.x as usize][i.y as usize] = new_building.clone();
            changed = true;
        }
        if changed {
            self.update_count();
        }


        return changed;
    }
}