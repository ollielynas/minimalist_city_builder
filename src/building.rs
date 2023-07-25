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

    /// the number of currently implemented stages
    pub fn no_stages() -> i32 {6}

    /// creates a new stage based on a `i32` number. returns an empty stage if the number is not valid
    /// # Example
    /// ```
    /// let stage = Stage::new(1);
    /// assert_eq!(stage.num, 1);
    /// ```
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
                    Carrot,
                ],
                5 => vec![
                    Bank,
                    Apartment,
                    FireStation,
                    PoliceStation,
                    Hospital,
                    FoodTruck,
                    Cpu,
                ],
                6 => vec![
                    Lightning,
                    Siren,
                    AirTrafficControl,
                    Runway,
                    StairsIntoTheVoid,
                    Garage,
                    LightHouse,
                    Lightbulb,
                    Mosque,
                    NuclearPowerPlant,
                    Rocket,
                    RobotFactory,
                    Cookie,
                    Database,
                    PalmTree,
                    Turret,
                ],
                _ => vec![],
            },
            // descriptions need updating
            description: match num {
                1 => "Plop down your house, and some crops, separated by at least one land tile".to_owned(),
                2 => "Build a warehouse and a battery.".to_owned(),
                3 => "The industrial revolution has arrived!. Factories can be used to operate a wide range of things, including steel mills and power plants.".to_owned(),
                4 => "Build a basic research facility, a concrete mixer, a gauge, and an asphalt plant.".to_owned(),
                5 => "Expand into a city, with asphalt instep of icky dirt (hint everything in a city needs road access)".to_owned(),
                6 => "TODO".to_owned(),
                a => format!("stage {a} has no description"),
            },
            title: match num {
                1 => "Just a simple framer".to_owned(),
                2 => "Power Up".to_owned(),
                3 => "Industrial Revolution".to_owned(),
                4 => "Research".to_owned(),
                5 => "City".to_owned(),
                6 => "TODO".to_owned(),
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
    Carrot,
    Hospital,
    FoodTruck,
    Lightning,
    Siren,
    AirTrafficControl,
    Runway,
    Cpu,
    StairsIntoTheVoid,
    Garage,
    LightHouse,
    Lightbulb,
    Mosque,
    NuclearPowerPlant,
    Rocket,
    RobotFactory,
    Cookie,
    Database,
    PalmTree,
    Turret,
    

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
            Carrot => egui_phosphor::CARROT,
            Lightning => egui_phosphor::LIGHTNING,
            Siren => egui_phosphor::SIREN,
            AirTrafficControl => egui_phosphor::AIR_TRAFFIC_CONTROL,
            Runway => egui_phosphor::AIRPLANE_IN_FLIGHT,
            Cpu => egui_phosphor::CPU,
            StairsIntoTheVoid => egui_phosphor::STAIRS,
            Garage => egui_phosphor::GARAGE,
            LightHouse => egui_phosphor::LIGHTHOUSE,
            Lightbulb => egui_phosphor::LIGHTBULB,
            Mosque => egui_phosphor::MOSQUE,
            NuclearPowerPlant => egui_phosphor::RADIOACTIVE,
            Rocket => egui_phosphor::ROCKET,
            RobotFactory => egui_phosphor::ROBOT,
            Cookie => egui_phosphor::COOKIE,
            Database => egui_phosphor::DATABASE,
            PalmTree => egui_phosphor::TREE_PALM,
            Turret => egui_phosphor::CASTLE_TURRET,


        }.to_owned()
    }
    /// returns the name of the building
    /// # Example
    /// ```
    /// let name = BuildingType::House.name();
    /// assert_eq!(name, "House");
    /// ```
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
            Carrot => "Carrot",
            Lightning => "Lightning",
            Siren => "Siren",
            AirTrafficControl => "Air Traffic Control",
            Runway => "Runway",
            Cpu => "Computational Research Facility",
            StairsIntoTheVoid => "Stairs Into The Void",
            Garage => "Garage",
            LightHouse => "Light House",
            Lightbulb => "Lightbulb",
            Mosque => "Mosque",
            NuclearPowerPlant => "Nuclear Power Plant",
            Rocket => "Rocket",
            RobotFactory => "Robot Factory",
            Cookie => "Cookie",
            Database => "Database",
            PalmTree => "Palm Tree",
            Turret => "Turret",

        }.to_owned()
    }
    /// returns the output of the building as a vector of tuples
    /// # Example
    /// ```
    /// let output = BuildingType::House.output();
    /// assert_eq!(output, vec![(Resource::PlaceholderResource, 0)]);
    /// ```

    pub fn output(&self) -> Vec<(Resource, i32)> {
        match self {
            Apartment => vec![(Resource::PlaceholderResource, 0)],
            Grain => vec![(Resource::Food, 1), (Resource::Seed, 1)],
            Carrot => vec![(Resource::Food, 3)],
            Tree => vec![(Resource::Wood, 1)],
            Shop => vec![(Resource::Tax, 2)],
            Warehouse => vec![(Resource::Storage, 100)],
            Battery => vec![],
            SteelProduction => vec![(Resource::Steel, 1)],
            Bank => vec![(Resource::CashStorage, 1000)],
            BasicResearchFacility => vec![(Resource::BasicScience, 1)],
            ConcreteMixer => vec![(Resource::Concrete, 10)],
            Cpu => vec![(Resource::Computation, 1)],
            Ground
            |House
            |Gauge
            |Asphalt
            |FireStation
            |PoliceStation
            |Hospital
            |Factory
            => vec![],// the following buildings have no output
            FoodTruck => vec![(Resource::Tax, 25)],
            Lightning
            |AirTrafficControl
            |Runway
            |StairsIntoTheVoid
            |Garage
            |LightHouse
            |Lightbulb
            |Mosque
            |NuclearPowerPlant
            |Rocket
            |RobotFactory
            |Cookie
            |Database
            |PalmTree
            |Turret
            |Siren
            => vec![(Resource::PlaceholderResource, 0)],
            
        }
    }
    /// returns the cost of the building as a vector of tuples
    /// # Example
    /// ```
    /// let cost = BuildingType::House.cost();
    /// assert_eq!(cost, vec![(Resource::Wood, 10), (Resource::Food, 10)]);
    /// ```
    pub fn cost(&self) -> Vec<(Resource, i32)> {
        let cost = match self {
            Carrot => vec![(Resource::Seed, 50)],
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
            FoodTruck => vec![(Resource::Food, 5000), (Resource::Wood, 1000)],
            Siren
            |LightHouse
            |Lightning
            |AirTrafficControl
            |Runway
            |Cpu
            |StairsIntoTheVoid
            |Garage
            |Lightbulb
            |Mosque
            |NuclearPowerPlant
            |Rocket
            |RobotFactory
            |Cookie
            |Database
            |PalmTree
            |Turret
            => vec![(Resource::PlaceholderResource, 0)],
        };
        return cost
    }
}


// list of all resources that a building can output
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
    Computation,
    PlaceholderResource,
}

impl Resource {
    /// returns the symbol of the resource
    /// # Example
    /// ```
    /// let symbol = Resource::Food.symbol();
    /// assert_eq!(symbol, egui_phosphor::HAMBURGER);
    /// ```

    pub fn symbol(&self) -> String {
        match self {
            Resource::Food => egui_phosphor::HAMBURGER,
            Resource::Tax => egui_phosphor::CURRENCY_DOLLAR,
            Resource::Wood => egui_phosphor::TREE_EVERGREEN,
            Resource::Seed => egui_phosphor::DROP,
            Resource::Storage => egui_phosphor::PACKAGE,
            Resource::Steel => egui_phosphor::PAPERCLIP,
            Resource::CashStorage => egui_phosphor::PIGGY_BANK,
            Resource::BasicScience => egui_phosphor::TEST_TUBE,
            Resource::Concrete => egui_phosphor::PAINT_BUCKET,
            Resource::Computation => egui_phosphor::CPU,
            Resource::PlaceholderResource => egui_phosphor::PLACEHOLDER,
        }.to_owned()
    }
    /// returns the name of the resource
    /// # Example
    /// ```
    /// let name = Resource::Food.name();
    /// assert_eq!(name, "Food");
    /// ```

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
            Resource::Computation => "Computation",
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
    /// the default value for a building is `BuildingType::Ground`
    fn default() -> Self {
        Building::new(&BuildingType::Ground)
    }
}



impl Building {
    /// create a new `Building` from a reference to a `BuildingType`
    /// # Example
    /// ```
    /// let building = Building::new(&BuildingType::House);
    /// assert_eq!(building.building_type, BuildingType::House);
    /// assert_eq!(building.cost, vec![(Resource::Wood, 10), (Resource::Food, 10)]);
    /// ```

    pub fn new(building_type: &BuildingType) -> Building {
        let mut required_adj = Vec::new();
        if [Battery, SteelProduction, ConcreteMixer].contains(&building_type) {required_adj.push(Factory)} // must be next to factory
        if [ConcreteMixer].contains(&building_type) {required_adj.push(Gauge)}

        if building_type == &BasicResearchFacility {required_adj.push(House); required_adj.push(Battery)}
        


        // these buildings can all be next to each other. I use this to make this process easer and less repetitive. 
        let city_tiles = [Bank, FireStation, PoliceStation, Hospital, Apartment, FoodTruck, Cpu];

        let mut optional_adj = Vec::new();

        if ![].contains(&building_type) {optional_adj.push(*building_type)} // cannot be next to self

        // if two buildings should be valid nex to each other than they can be put as a tuple here. 
        for i in [
            (Warehouse, Shop),
            (Battery, Factory),
            (SteelProduction, Factory),
            (House, BasicResearchFacility),
            (BasicResearchFacility, Battery),
            (Factory, ConcreteMixer),
            (ConcreteMixer, Gauge),
            (Grain, Carrot),
            ] {
            if &i.0 == building_type {optional_adj.push(i.1)}
            if &i.1 == building_type {optional_adj.push(i.0)}
        }

        // implement the logic for all city tiles. 
        if city_tiles.contains(&building_type) {
            for i in city_tiles.iter() {
                optional_adj.push(*i)
            }
            required_adj.push(Asphalt)
        }
        if building_type == &BuildingType::Asphalt {
            optional_adj.append(city_tiles.to_vec().as_mut())
        }

        // this list is a subset of the city tiles. it contains all the tiles that can produce resources.
        // this is useful because all of these buildings need to have access to an apartment building. 
        let production_city_tiles = [Bank, FoodTruck, Cpu];

    

    
        let mut tile_adj = match building_type { // mut be within one tile of:
            Shop => vec![Grain, House, Tree],
            Grain|Tree|Carrot => vec![House],
            Factory => vec![],
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
            |Siren
            |LightHouse
            |Lightning
            |AirTrafficControl
            |Runway
            |Cpu
            |StairsIntoTheVoid
            |Garage
            |Lightbulb
            |Mosque
            |NuclearPowerPlant
            |Rocket
            |RobotFactory
            |Cookie
            |Database
            |PalmTree
            |Turret
            => vec![],
        };

        if production_city_tiles.contains(&building_type) {
            tile_adj.push(Apartment)
        }
        
        



        Building {
            building_type: *building_type,
            required_adj,
            optional_adj,
            tile_adj,
            cost: building_type.cost(),
            symbol: building_type.symbol(),
        }
    }





            
    //     }
    // }
}
