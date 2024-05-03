use std::collections::HashMap;
use web_sugars::prelude::*;

use glam::Vec2;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

use crate::{map_features::MapFeature, road::Road, zone::{self, Zone}};

lazy_static! {
    pub static ref CITY: Mutex<City> = Mutex::new(City::new());
}



pub struct City {
    pub points: HashMap<u64, Vec2>,
    pub roads: HashMap<u64, Road>,
    pub zones: HashMap<u64, Zone>,
}


impl City {


    fn new() -> City {
        City {
            points: HashMap::new(),
            roads: HashMap::new(),
            zones: HashMap::new(),
        }
    }


    pub fn update(&mut self) {

    let road_len = self.roads.len();
    let zone_len = self.zones.len();

    for (i, (_, road)) in self.roads.iter_mut().enumerate() {
        if road.index == 999999 {
            road.index = road_len;
            road.name += &" ".to_owned();
            road.name +=  &road_len.to_string();
        }
    }
    for (i, (_, zone)) in self.zones.iter_mut().enumerate() {
        if zone.index == 999999 {
            zone.index = zone_len;
            zone.name += &" ".to_owned();
            zone.name +=  &zone_len.to_string();
        }
    }

    let elements = get_element_by_id("map").unwrap().get_elements_by_class_name("point");
    
    for i in 0..elements.length() {
        let element = elements.get_with_index(i).unwrap();
        let id = element.id().replace("id", "").parse::<u64>().unwrap();

        self.points.insert(id, Vec2::new(
            element.get_attribute("x").unwrap().parse::<f32>().unwrap(), 
            element.get_attribute("y").unwrap().parse::<f32>().unwrap()
        ));
    }

    let mut elem_inner_html = "".to_owned();

    

    let mut road_list:Vec<&Road> = self.roads.values().collect::<Vec<_>>();
    road_list.sort_by(|a,b| a.index.cmp(&b.index));

    

    let mut zone_list:Vec<&Zone> = self.zones.values().collect::<Vec<_>>();
    zone_list.sort_by(|a,b| a.index.cmp(&b.index));

    for road in road_list {
        elem_inner_html += &road.html();
    }
    for zone in zone_list {
        elem_inner_html += &zone.html();
    }
    
    

    let mut svg_inner_html = "".to_owned();
    for zone in &self.zones {
        svg_inner_html += &zone.1.svg(&self.points);
    }
    for road in &self.roads {
        svg_inner_html += &road.1.svg(&self.points);
    }

    let inner_before = get_element_by_id("elem").unwrap().inner_html().replace("open=\"\"", "");

    

    get_element_by_id("elem").unwrap().set_inner_html(&elem_inner_html);
    get_element_by_id("preview_svg").unwrap().set_inner_html(&svg_inner_html);

    }


    

}



