use std::collections::HashMap;

use glam::Vec2;
use strum::IntoEnumIterator;

pub trait MapFeatureType: IntoEnumIterator + PartialEq + Default {


    fn kind() -> &'static str;


    fn name(&self) -> String;

    fn html_selector(&self, id: u64) -> String {
        let names = Self::iter()
            .map(|x| x.name())
            .collect::<Vec<String>>();

        let mut s = 
        format!("<select class='type' name='type' id='type' onchange='this.parentElement.parentElement.setAttribute(\"type\", this.value);add_{}(\"id{id}\")'>",
        Self::kind(),
    );
        for name in names {
            
            s += &format!("<option id = 'type_dropdownid{id}' {} value=\"{name}\">{name}</option>",
            if name == self.name() {
                "selected"
            }else {""}
        );
        }
        s += "</select>";
        return s;
    }
}

pub trait MapFeature {
    type FeatureType: MapFeatureType;
    
    

    fn new(id: u64, name: String, points: Vec<u64>, type_: Self::FeatureType, index: usize) -> Self;

    fn index(&self) -> usize;

    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn points(&self) -> &Vec<u64>;
    fn feature_type(&self) -> Self::FeatureType;

    fn from_id(id: u64, index: usize) -> Self;

    fn svg(&self, points: &HashMap<u64, Vec2>) -> String;

    fn html(&self) -> String {
        let id = self.id();
        let name = self.name();
        let points = self
            .points()
            .iter()
            .map(|x| format!("-{x}-"))
            .collect::<String>();
        format!("
        <div name = '' type='{}' index={} points='{points}' class='{} feature' id='id{id}'>
        <div oninput='this.parentElement.setAttribute(\"name\", this.innerHTML)' onclick=\"listenForDoubleClick(this);\" onblur=\"this.contentEditable=false;\" class='name'>{name}</div>
        <details>
        <summary>edit</summary>
        <button onclick=\"document.body.setAttribute('input_type','connect_points');document.body.setAttribute('selected_id','id{id}')\">set points</button>
        <button onclick=\"remove_{}(\'id{id}\')\">delate</button>
        {}
        </details>
        </div>",
        self.feature_type().name(),
       self.index(),
        Self::FeatureType::kind(),
        Self::FeatureType::kind(),
        self.feature_type().html_selector(self.id())
    )
    }
}
