

use std::collections::HashMap;

use glam::Vec2;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::map_features::{MapFeature, MapFeatureType};

#[derive(Debug, EnumIter, PartialEq, Copy, Clone)]
pub enum RoadType {
    Street,
    Highway,
}

impl Default for RoadType {
    fn default() -> Self {
        RoadType::Street
    }
}

impl MapFeatureType for RoadType {
    fn name(&self) -> String {
        match self {
            RoadType::Street => "Street",
            RoadType::Highway => "Highway",
        }.to_owned()
    }

    fn kind() -> &'static str {
        "road"
    }
}

impl RoadType {

    pub fn name(&self) -> String {
        match self {
            RoadType::Street => "Street",
            RoadType::Highway => "Highway",
        }.to_owned()
    }

}

pub struct Road {
    pub type_: RoadType,
    pub points: Vec<u64>,
    pub id: u64,
    pub name: String,
    pub index: usize,
}

impl MapFeature for Road {
    type FeatureType = RoadType;

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
        Road { type_: RoadType::Street, points: vec![], id:  id, name: "Road".to_owned(), index }
    }

    fn new(id: u64, name: String, points: Vec<u64>, type_: Self::FeatureType, index: usize) -> Self {
        Road { type_, points, id, name, index }
    }

    fn svg(&self, point_hashmap: &HashMap<u64, Vec2>) -> String {
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
    
    
        let mut svg = "".to_owned();
        match self.type_ {
            RoadType::Street => {
                svg += &format!("<path id='pathID{}' class='street' d=\"{path}\"/>\n",self.id);
            },
            RoadType::Highway => {
                svg += &format!("<path id='pathID{}' class='highway' d=\"{path}\"/>", self.id);
            },
        };

        svg += &format!("
        <text>
            <textPath dominant-baseline=\"middle\" text-anchor=\"middle\" href=\"#pathID{}\" textLength=\"100%\" startOffset=\"50%\">{}</textPath>
        </text>",
        self.id,
        self.name,
    );


    
        return svg;
        
    }
}


impl Road {

}