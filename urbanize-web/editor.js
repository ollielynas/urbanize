


let map = document.querySelector(".map");

console.log(map);
let old_input_state = ""
let before_selected_points = ""


function listenForDoubleClick(element) {
    element.contentEditable = true;
    setTimeout(function() {
      if (document.activeElement !== element) {
        element.contentEditable = false;
      }
    }, 300);
  }

function save_points() {
    
    let id = document.body.getAttribute("selected_id");

    let elem = document.getElementById(id);
    elem.setAttribute("points", document.body.getAttribute("selected_points"));

    if (elem.classList.contains("road")) {
        add_road(id);
    }
    if (elem.classList.contains("zone")) {
        add_zone(id);
        
    }

    console.log("set points of ",id);
    document.body.setAttribute("selected_points", "");
    document.body.setAttribute("selected_id", "");

    update();
}

document.body.addEventListener("click", (e) => {
    let new_input_state = document.body.getAttribute("input_type");
    console.log(new_input_state,old_input_state)
    let new_state = (new_input_state == old_input_state);
    if (!new_state) {
        old_input_state = new_input_state;
        console.log("not_new");
        return
    }

    if (old_input_state == "connect_points") {
        
    } 

    if (new_input_state == "connect_points") {
        let points = document.querySelectorAll(".point");
        let ids = document.body.getAttribute("selected_points");

        var div_array = [...points]; // converts NodeList to Array
        div_array.forEach(div => {

        // do something awesome with each div
        let id = div.id;
        if (ids.includes(id)) {
            div.style.backgroundColor  = "red";
            console.log("red")
        }else {
            div.style.backgroundColor  = "white";
        }
        console.log(div, ids, id);
        });

    }

    old_input_state = new_input_state;
});

map.addEventListener("click", (e) => {
    const target = e.target;

    // Get the bounding rectangle of target
    const rect = target.getBoundingClientRect();
    
    // Mouse position
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    if (document.body.getAttribute("input_type") == "add_point" && !e.target.classList.contains("point")) {
        console.log("click");
        let point = document.createElement("div");
        // console.log(point);
        point.classList.add("point");
        point.style.left = x.toString() + " px";
        point.style.top = y.toString() + " px";
        console.log("left: "+ x.toString() + "px;" +"top: "+ y.toString() + "px;");
        point.style.cssText = ("left: "+ x.toString() + "px;" +"top: "+ y.toString() + "px;");
        point.setAttribute("x", x.toString());
        point.setAttribute("y", y.toString());

        point.id = "id"+ Math.round(Math.random() * 10000000).toString();
        map.appendChild(point);
        // point.classList.
        console.log(point);
    }

    if (document.body.getAttribute("input_type") == "connect_points" 
    && e.target.classList.contains("point")
    && !document.body.getAttribute("selected_points").includes("-"+e.target.id+"-")) {
        console.log(e.target.id);
        document.body.setAttribute("selected_points", document.body.getAttribute("selected_points")+"-"+e.target.id+"-");
    }
});