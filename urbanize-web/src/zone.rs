use std::collections::HashMap;

use glam::Vec2;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::map_features::{MapFeature, MapFeatureType};



const TOTAL_TRAFFIC: f32 = 1.0/625.0;

const TRAFFIC_TIME: [f32; 24] = [
    50.0/TOTAL_TRAFFIC,
    45.0/TOTAL_TRAFFIC,
    25.0/TOTAL_TRAFFIC,
    20.0/TOTAL_TRAFFIC,
    20.0/TOTAL_TRAFFIC,
    25.0/TOTAL_TRAFFIC,
    50.0/TOTAL_TRAFFIC,
    200.0/TOTAL_TRAFFIC,
    600.0/TOTAL_TRAFFIC,
    550.0/TOTAL_TRAFFIC,
    620.0/TOTAL_TRAFFIC,
    600.0/TOTAL_TRAFFIC,
    500.0/TOTAL_TRAFFIC,
    350.0/TOTAL_TRAFFIC,
    200.0/TOTAL_TRAFFIC,
    -400.0/TOTAL_TRAFFIC,
    -510.0/TOTAL_TRAFFIC,
    -500.0/TOTAL_TRAFFIC,
    -620.0/TOTAL_TRAFFIC,
    -400.0/TOTAL_TRAFFIC,
    -300.0/TOTAL_TRAFFIC,
    -250.0/TOTAL_TRAFFIC,
    -150.0/TOTAL_TRAFFIC,
    -100.0/TOTAL_TRAFFIC,
];


pub struct Zone {
    pub type_ : ZoneType,

    /// this is the number of people who are trying to get here/leave here in the given hour
    /// used for traffic simulation
    // pub delta_people: [f32; 24],

    pub points: Vec<u64>,
    pub id: u64,
    pub index: usize,
    pub name: String,
}

impl MapFeature for Zone {
    type FeatureType = ZoneType;

    fn new(id: u64, name: String, points: Vec<u64>, type_: Self::FeatureType, index: usize) -> Self {
        Zone { type_, points, id, name, index }
    }

    fn index(&self) -> usize {
        self.index
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn points(&self) -> &Vec<u64> {
        &self.points
    }

    fn feature_type(&self) -> Self::FeatureType {
        self.type_
    }


    fn from_id(id: u64, index: usize) -> Self {
        Zone { type_: ZoneType::default(), points: vec![], id: id, name: "Zone".to_owned(),
        index
    }
    }

    
    fn svg(&self, point_hashmap: &HashMap<u64, Vec2>) -> String {
        
        let mut average = Vec2::MAX;

        if self.points.len() > 2 {
            average = *point_hashmap.get(&self.points[0]).unwrap();
            for i in 1..self.points.len() {
                average += *point_hashmap.get(&self.points[i]).unwrap();
            }
            average /= self.points.len() as f32;
        }

        let mut path = "".to_owned();
        let mut first = true;
        for point in &self.points {
            let v = point_hashmap.get(point).unwrap();
            let x = v.x;
            let y = v.y;
    
            if first {
                first = false;
                path += &format!("M {x} {y} ");
            }else {
                path += &format!("L {x} {y} ");
            }
        }

        path += " z";

        
    
    
        let mut svg = "".to_owned();


        
        
        
        match self.type_ {
            ZoneType::Commercial => {
                svg += &format!("<path class='area_path commercial' d=\"{path}\"/>\n");
                
            },
            ZoneType::Park => {
                svg += &format!("<path class='area_path park' d=\"{path}\"/>\n");
                
            },
            ZoneType::Residential => {
                svg += &format!("<path class='area_path residential' d=\"{path}\"/>\n");
            },
            ZoneType::Industrial => {
                svg += &format!("<path class='area_path' industrial d=\"{path}\"/>\n");
            },
        };
        
        if self.points.len() > 0 {
        svg += &format!("<text x=\"{}px\" y=\"{}px\" dominant-baseline=\"middle\" text-anchor=\"middle\">{}\n({})</text>",
        average.x,
        average.y,
        self.type_.name(), self.name);
        }
        return svg;
        
    }
    
}


impl Default for Zone {
    fn default() -> Self {
        Zone {
            type_: ZoneType::Park,
            // delta_people: TRAFFIC_TIME,
            points: vec![],
            id: fastrand::u64(0..1000000),
            name: "name".to_owned(),
            index: 0,
        }
    }
}



#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum ZoneType {
    Park,
    Residential,
    Commercial,
    Industrial,

}

impl MapFeatureType for ZoneType {
    fn name(&self) -> String {
        match self {
            ZoneType::Park => "Park".to_owned(),
            ZoneType::Residential => "Residential".to_owned(),
            ZoneType::Commercial => "Commercial".to_owned(),
            ZoneType::Industrial => "Industrial".to_owned(),
        }.to_owned()
    }

    fn kind() -> &'static str {
        "zone"
    }
}

impl Default for ZoneType {
    fn default() -> Self {
        ZoneType::Park
    }
}

impl ZoneType {
    
}