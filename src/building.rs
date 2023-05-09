use std::vec;
use strum::EnumIter;
use crate::BuildingType::*;

use strum::IntoEnumIterator;

#[derive(Savefile)]

pub struct Stage {
    pub num: i32,
    pub buildings: Vec<BuildingType>,
    pub title: String,
    pub description: String,
    pub enabled: bool,
    pub unlock_at: Vec<(Resource, i32)>
}

impl Stage {
    
    pub fn new(num: i32) -> Stage {
        Stage {
            num,
            buildings: match num {
                1 => vec![
                    House,
                    Grain,
                ],
                2 => vec![
                    Tree,
                    Shop,
                    Warehouse,
                ],
                3 => vec![
                    Factory,
                    Battery,
                    SteelProduction,
                ],
                4 => vec![
                    BasicResearchFacility,
                    ConcreteMixer,
                    Gauge,
                    Asphalt,
                ],
                5 => vec![
                    Bank,
                    Apartment,
                    FireStation,
                    PoliceStation,
                    Hospital,
                    FoodTruck,
                ],
                _ => vec![],
            },
            /// descriptions need updating
            description: match num {
                1 => "Plop down your house, and some crops, separated by at least one land tile".to_owned(),
                2 => "Build a warehouse and a battery.".to_owned(),
                3 => "The industrial revolution has arrived!. Factories can be used to operate a wide range of things, including steel mills and power plants.".to_owned(),
                4 => "Build a basic research facility, a concrete mixer, a gauge, and an asphalt plant.".to_owned(),
                5 => "Expand into a city, with asphalt instep of icky dirt (hint everything in a city needs road access)".to_owned(),
                a => format!("stage {a} has no description"),
            },
            title: match num {
                1 => "Just a simple framer".to_owned(),
                2 => "Power Up".to_owned(),
                3 => "Industrial Revolution".to_owned(),
                4 => "Research".to_owned(),
                5 => "City".to_owned(),
                a => format!("stage {a} has no title"),
            },
            enabled: num == 1,
            unlock_at: match num {
                1 => vec![],
                2 => vec![(Resource::Seed, 50)],
                3 => vec![(Resource::Wood, 100)],
                4 => vec![(Resource::Storage, 200)],
                5 => vec![(Resource::Concrete, 50)],
                _ => Resource::iter().map(|x| (x, 999999)).collect::<Vec<(Resource, i32)>>(),
            },
        }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone, EnumIter, Savefile)]
pub enum BuildingType {
    Ground,
    House,
    Grain,
    Tree,
    Shop,
    Warehouse,
    Battery,
    Factory,
    SteelProduction,
    Bank,
    BasicResearchFacility,
    ConcreteMixer,
    Gauge,
    Asphalt,
    Apartment,
    FireStation,
    PoliceStation,
    Hospital,
    FoodTruck,
}

impl BuildingType {
    pub fn symbol(&self) -> String {
        match self {
            Ground => "  ",
            Apartment => egui_phosphor::BUILDINGS,
            Tree => egui_phosphor::TREE,
            House => egui_phosphor::HOUSE,
            Grain => egui_phosphor::GRAINS,
            Shop => egui_phosphor::STOREFRONT,
            Warehouse => egui_phosphor::WAREHOUSE,
            Battery => egui_phosphor::BATTERY_CHARGING_VERTICAL,
            Factory => egui_phosphor::FACTORY,
            SteelProduction => egui_phosphor::BARCODE,
            Bank => egui_phosphor::BANK,
            BasicResearchFacility => egui_phosphor::CIRCUITRY,
            ConcreteMixer => egui_phosphor::HOURGLASS_MEDIUM,
            Gauge => egui_phosphor::GAUGE,
            Asphalt => egui_phosphor::SQUARE,
            FireStation => egui_phosphor::FIRE_EXTINGUISHER,
            PoliceStation => egui_phosphor::POLICE_CAR,
            Hospital => egui_phosphor::FIRST_AID_KIT,
            FoodTruck => egui_phosphor::VAN,

        }.to_owned()
    }

    pub fn name(&self) -> String {
        match self {
            Apartment => "Apartment",
            Ground => "Ground",
            Tree => "Tree",
            House => "House",
            Grain => "Grain",
            Shop => "Shop",
            Battery => "Battery",
            Warehouse => "Warehouse",
            Factory => "Factory",
            SteelProduction => "Steel Mill",
            Bank => "Bank",
            BasicResearchFacility => "Basic Research Facility",
            ConcreteMixer => "Concrete Mixer",
            Gauge => "Gauge",
            Asphalt => "Asphalt",
            FireStation => "Fire Station",
            PoliceStation => "Police Station",
            Hospital => "Hospital",
            FoodTruck => "Food Truck",
        }.to_owned()
    }

    pub fn output(&self) -> Vec<(Resource, i32)> {
        match self {
            Apartment => vec![(Resource::PlaceholderResource, 0)],
            Grain => vec![(Resource::Food, 1), (Resource::Seed, 1)],
            Tree => vec![(Resource::Wood, 1)],
            Shop => vec![(Resource::Tax, 2)],
            Warehouse => vec![(Resource::Storage, 100)],
            Battery => vec![],
            SteelProduction => vec![(Resource::Steel, 1)],
            Bank => vec![(Resource::CashStorage, 1000)],
            BasicResearchFacility => vec![(Resource::BasicScience, 1)],
            ConcreteMixer => vec![(Resource::Concrete, 10)],
            Ground
            |House
            |Gauge
            |Asphalt
            |FireStation
            |PoliceStation
            |Hospital
            |Factory
             => vec![],
            FoodTruck => vec![(Resource::Tax, 25)],
            
        }
    }
}



#[derive(PartialEq, Eq, Hash, Copy, Clone, EnumIter, Savefile)]
pub enum Resource {
    Food,
    Tax,
    Wood,
    Seed,
    Storage,
    CashStorage,
    Steel,
    BasicScience,
    Concrete,
    PlaceholderResource,
}

impl Resource {
    pub fn symbol(&self) -> String {
        match self {
            Resource::Food => egui_phosphor::HAMBURGER,
            Resource::Tax => egui_phosphor::HAND_COINS,
            Resource::Wood => egui_phosphor::TREE_EVERGREEN,
            Resource::Seed => egui_phosphor::DROP,
            Resource::Storage => egui_phosphor::PACKAGE,
            Resource::Steel => egui_phosphor::PAPERCLIP,
            Resource::CashStorage => egui_phosphor::PIGGY_BANK,
            Resource::BasicScience => egui_phosphor::TEST_TUBE,
            Resource::Concrete => egui_phosphor::PAINT_BUCKET,
            Resource::PlaceholderResource => egui_phosphor::PLACEHOLDER,
        }.to_owned()
    }
    pub fn name(&self) -> String {
        match self {
            Resource::Food => "Food",
            Resource::Tax => "Tax",
            Resource::Wood => "Wood",
            Resource::Seed => "Seeds",
            Resource::Storage => "Storage",
            Resource::Steel => "Steel",
            Resource::CashStorage => "Cash Storage",
            Resource::BasicScience => "Basic Science",
            Resource::Concrete => "Concrete",
            Resource::PlaceholderResource => "Placeholder Resource",
        }.to_owned()
    }
}




#[derive(PartialEq, Eq, Hash, Clone, Savefile)]
pub struct Building {
    pub building_type: BuildingType,
    pub required_adj: Vec<BuildingType>,
    pub optional_adj: Vec<BuildingType>,
    pub tile_adj: Vec<BuildingType>,
    pub cost: Vec<(Resource, i32)>,
    pub symbol: String,
}


impl Default for Building {
    fn default() -> Self {
        Building::new(BuildingType::Ground)
    }
}



impl Building {
    pub fn new(building_type: BuildingType) -> Building {

        let mut required_adj = Vec::new();
        if [Battery, SteelProduction, ConcreteMixer].contains(&building_type) {required_adj.push(Factory)} // must be next to factory
        if [ConcreteMixer].contains(&building_type) {required_adj.push(Gauge)} // must be next to battery

        if building_type == BuildingType::BasicResearchFacility {
            required_adj.push(Battery);
            required_adj.push(House)
        }
        



        let city_tiles = [Bank, FireStation, PoliceStation, Hospital, Apartment, FoodTruck];

        let mut optional_adj = Vec::new();
        if ![BasicResearchFacility].contains(&building_type) && !city_tiles.contains(&building_type) {optional_adj.push(Ground)} // cannot be next to ground

        if ![].contains(&building_type) {optional_adj.push(building_type)} // cannot be next to self

        for i in [(Warehouse, Shop), (Battery, Factory)] {
            if i.0 == building_type {optional_adj.push(i.1)}
            if i.1 == building_type {optional_adj.push(i.0)}
        }

        // city tiles 
        if city_tiles.contains(&building_type) {
            for i in city_tiles.iter() {
                optional_adj.push(*i)
            }
            required_adj.push(Asphalt)
        }

        let production_city_tiles = [Bank, FoodTruck];

        




        let mut tile_adj = match building_type { // mut be within one tile of:
            Shop => vec![Grain, House, Tree],
            Grain|Tree => vec![House],
            Factory => vec![Warehouse],
            Apartment => vec![FireStation, Hospital, PoliceStation],
            Ground|House|Warehouse|Battery|SteelProduction => Vec::new(),
            BasicResearchFacility
            |Gauge
            |ConcreteMixer
            |Asphalt
            |Bank
            |FireStation
            |PoliceStation
            |Hospital
            |FoodTruck

            => vec![],
        };

        if production_city_tiles.contains(&building_type) {
            tile_adj.push(Apartment)

        }

        let cost = match building_type {
            Asphalt => vec![(Resource::Concrete, 1)],
            Apartment => vec![(Resource::Food, 1), (Resource::Concrete, 50), (Resource::Steel, 10)],
            Shop => vec![(Resource::Wood, 50), (Resource::Food, 50)],
            Ground => vec![],
            House => vec![(Resource::Wood, 10), (Resource::Food, 10)],
            Grain|Tree => vec![(Resource::Seed, 5)],
            Warehouse => vec![(Resource::Wood, 100)],
            Battery => vec![(Resource::Steel, 20), (Resource::Food, 200)],
            Factory => vec![(Resource::Wood, 100), (Resource::Food, 100), (Resource::Seed, 100)], // placeholder
            SteelProduction => vec![(Resource::Wood, 150)],
            Bank => vec![(Resource::Wood, 200), (Resource::Food, 200), (Resource::Steel, 30), (Resource::Tax, 300)],
            BasicResearchFacility => vec![(Resource::Wood, 100), (Resource::Food, 100), (Resource::Seed, 100), (Resource::Steel, 100)],
            ConcreteMixer => vec![(Resource::Steel, 100), (Resource::BasicScience, 100)],
            Gauge => vec![(Resource::Steel, 50), (Resource::BasicScience, 300)],
            FireStation => vec![(Resource::Concrete, 500), (Resource::Steel, 20)],
            Hospital => vec![(Resource::Concrete, 1000), (Resource::Food, 1500), (Resource::BasicScience, 50)],
            PoliceStation => vec![(Resource::Concrete, 500), (Resource::Food, 500)],
            FoodTruck => vec![(Resource::Food, 2000), (Resource::Wood, 700)],
        };



        Building {
            building_type,
            required_adj,
            optional_adj,
            tile_adj,
            cost,
            symbol: building_type.symbol(),
        }
    }
}
