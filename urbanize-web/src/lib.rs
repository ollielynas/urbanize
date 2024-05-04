
mod city;
mod road;
mod zone;
mod map_features;
use std::panic;

use city::CITY;
use map_features::MapFeature;
use road::{Road, RoadType};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use web_sugars::{document, prelude::*};
use web_sys::Element;
use console_error_panic_hook;
use zone::{Zone, ZoneType};
use crate::map_features::MapFeatureType;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Zone::default();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");


    Ok(())
}

#[wasm_bindgen]
pub fn set_points(s: String, id: String) {

}

#[wasm_bindgen]
pub fn remove_road(id: String) -> Result<(), JsError> {
    let node = get_element_by_id(&id)?;
    node.parent_node().unwrap().remove_child(&node).unwrap();

    let id_int: u64 = id.replace("id", "").parse().unwrap();
    while CITY.try_lock().is_err() {
        
    }
    CITY.lock().unwrap().roads.remove(&id_int);
    Ok(())
}
#[wasm_bindgen]
pub fn remove_zone(id: String) -> Result<(), JsError> {
    let node = get_element_by_id(&id)?;
    node.parent_node().unwrap().remove_child(&node).unwrap();

    let id_int: u64 = id.replace("id", "").parse().unwrap();
    while CITY.try_lock().is_err() {
        
    }
    CITY.lock().unwrap().roads.remove(&id_int);
    Ok(())
}

pub fn add_thing<Feature: MapFeature>(id: String) -> Result<Feature, JsError> {
    if get_element_by_id(&id).is_err() {
        get_element_by_id("elem")?.insert_adjacent_html("beforeend", 
        /*&Feature {
            type_: Feature::,
            points: vec![],
            name: format!("Road #{}",get_element_by_id("elem")?.inner_html().matches("class=\"road\" id=\"id").count()+1),
            id: id.replace("id", "").parse::<u64>().unwrap()}*/
        &Feature::from_id(id.replace("id", "").parse::<u64>().unwrap(),
            999999
    ).html()
    ).unwrap();
    }

    // panic!("{:?}", get_body());

    // let e = get_element_by_id(&id)?;
    // panic!("\"{}\"",id);
    // panic!("{id}");

    let e = get_element_by_id(&id).unwrap();
    let name = e.get_elements_by_class_name("name").get_with_index(0).unwrap().inner_html();

    let id_int = id.replace("id", "").parse::<u64>().unwrap();
    
    let points = e.get_attribute("points").unwrap()
    .replace("id", "")
    .replace("--", "_")
    .replace("-", "")
    .split("_")
    .filter_map(|x| x.parse::<u64>().ok())
    .collect::<Vec<u64>>();
// panic!("{:?}", e.get_elements_by_class_name("type"));
    let dropdown: String = e.get_attribute("type").unwrap();
    let index = e.get_attribute("index").unwrap().parse::<usize>().unwrap();



// panic!("{}",dropdown);
    let mut type_ = Feature::FeatureType::default();

    for t in Feature::FeatureType::iter() {
        if t.name() == dropdown {
            type_ = t;
            // panic!("worked!! {dropdown}")
        }
    }




    return Ok(MapFeature::new(id_int, name, points, type_, index));
}


#[wasm_bindgen]
pub fn add_road(id: String) -> Result<(), JsError> {

    let road = match add_thing::<Road>(id) {
    Ok(a) => a,
    Err(_) => panic!("add thing error"),
    };

    CITY.lock().unwrap().roads.insert(road.id, road);
    CITY.lock().unwrap().update();

    return Ok(());
}
#[wasm_bindgen]
pub fn add_zone(id: String) -> Result<(), JsError> {

    let zone = match add_thing::<Zone>(id) {
    Ok(a) => a,
    Err(_) => panic!("add thing error"),
    };

    CITY.lock().unwrap().zones.insert(zone.id, zone);
    return Ok(());
}


#[wasm_bindgen]
pub fn update() {
    CITY.lock().unwrap().update();
}

