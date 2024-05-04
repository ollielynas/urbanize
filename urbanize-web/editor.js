


let map = document.querySelector(".map");
let map_parent = document.querySelector(".map").parentElement;

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
    let x = e.clientX - rect.left;
    let y = e.clientY - rect.top;
    let scale = parseFloat(map.style.scale);
    x /= scale;
    y /= scale;

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

map.addEventListener("scroll ", (e) => {
    console.log("x");
});

let leftValue = 0;
let topValue = 0;

// const map = document.querySelector(".map");
function onMouseDrag({ movementX, movementY }) {
    // let getContainerStyle = window.getComputedStyle(map);
    // let leftValue = parseInt(getContainerStyle.left);
    // let topValue = parseInt(getContainerStyle.top);
    leftValue += movementX
    topValue += movementY

    map.animate({
        left: `${leftValue}px`,
        top: `${topValue}px`
      }, { duration: 0, fill: "forwards" });

    // map.style.left = `${leftValue + movementX}px`;
    // map.style.top = `${topValue + movementY}px`;
}    

map.parentElement.addEventListener("mousedown", () => {
    let input =  document.body.getAttribute("input_type");
    if (input == "move_map") {
        map.parentElement.addEventListener("mousemove", onMouseDrag);
    }
});    
map.parentElement.addEventListener("wheel", (e) => {

    var rect = map.parentElement.getBoundingClientRect();
        console.log(rect);
      var x = e.clientX - rect.left - rect.width/2; //x position within the element.
      var y = e.clientY - rect.top - rect.height/2;

      console.log(x,y)

    zoom(e.deltaY/850.0, x, y);
});    
document.addEventListener("mouseup", () => {
    map.parentElement.removeEventListener("mousemove", onMouseDrag);
});    

function zoom(i, x,y) {
    let scale = parseFloat(map.style.scale);
    scale += scale*i;

    scale = Math.max(Math.min(15, scale), 0.01);
    map.style.scale = scale.toString();


    onMouseDrag({ movementX: (leftValue - x) * i, movementY: (topValue - y) * i});
}




function zoom_in() {
    zoom(1/10, 0,0);
}
function zoom_out() {
    zoom(-1/10,0,0);
}

