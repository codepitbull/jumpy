use quick_xml::Reader;
use quick_xml::events::Event;
use std::str;
use std::collections::HashMap;
use quick_xml::events::BytesStart;

//https://doc.mapeditor.org/en/stable/reference/tmx-map-format/

const FLIPPED_HORIZONTALLY_FLAG:i64 = 0x80000000;
const FLIPPED_VERTICALLY_FLAG:i64 = 0x40000000;
const FLIPPED_DIAGONALLY_FLAG:i64 = 0x20000000;
const CLEAR_MASK:i64 = !(FLIPPED_VERTICALLY_FLAG | FLIPPED_HORIZONTALLY_FLAG | FLIPPED_DIAGONALLY_FLAG);

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Renderorder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum StaggerAxis {
    X,
    Y
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum StaggerIndex {
    Even,
    Odd
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum PropertyType {
    String,
    Int,
    Float,
    Bool,
    Color,
    File
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum DrawOrder {
    Index,
    TopDown
}

#[derive(Debug)]
pub struct Tileset {
    pub firstgid: i64,
    pub source: Option<String>,
    pub spacing: i32,
    pub margin: i32,
    pub name: String,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub tilecount: i32,
    pub columns: i32,
    pub tiles: Vec<Tile>,
    pub tileoffset: Option<TileOffset>,
}

#[derive(Debug)]
pub struct Tile {
    pub id: i64,
    pub tile_type: Option<String>,
    pub probability: u32,
    pub terrain: Option<String>,
    pub image: Option<Image>,
}

#[derive(Debug)]
pub struct TileOffset {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct Layer {
    pub id: i64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub opacity: f32,
    pub visible: bool,
    pub offsetx: i32,
    pub offsety: i32,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub prop_type: PropertyType,
    pub value: String,
}

#[derive(Debug)]
pub struct Objectgroup {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    pub visible: bool,
    pub offsetx: i32,
    pub offsety: i32,
    pub draworder: DrawOrder,
    pub objects: Vec<Object>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug)]
pub struct Object {
    pub id: i64,
    pub name: Option<String>,
    pub object_type: Option<String>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub gid: Option<i64>,
    pub visible: bool,
    pub template: Option<String>,
    pub properties: Option<Vec<Property>>,
    pub flipped_horizontally: bool,
    pub flipped_vertically: bool,
    pub flipped_diagonally: bool,
}

#[derive(Debug)]
pub struct Imagelayer {
    pub id: i64,
    pub name: String,
    pub offsetx: i32,
    pub offsety: i32,
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    pub visible: bool,

}

#[derive(Debug)]
pub struct Image {
    pub format: Option<String>,
    pub source: String,
    pub trans: Option<String>,
    pub width: i32,
    pub height: Option<i32>,
}

#[derive(Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub offsetx: i32,
    pub offsety: i32,
    pub opacity: f32,
    pub visible: bool,
}

#[derive(Debug)]
pub struct Map {
    pub version: String,
    pub orientation: Orientation,
    pub renderorder: Renderorder,
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub backgroundcolor: String,
    pub nextobjectid: i32,
    pub staggeraxis: StaggerAxis,
    pub staggerindex: StaggerIndex,
    pub tileset: Tileset,
    pub objectgroups: Vec<Objectgroup>,
    pub properties: Option<Vec<Property>>
    //infinite ??? desert.tmx
}

impl Map {
    pub fn new(xml: &str) -> Map {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf = Vec::new();

        let mut map: Option<Map> = None;

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"map" => {
                            map = Some(read_map(e, &mut reader));
                        },
                        _ => (),
                    }
                },
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        //TODO this can be better
        map.unwrap()
    }
}

fn propertytype_from_string(propertytype: &String) -> PropertyType {
    match propertytype.to_lowercase().as_ref() {
        "string" => PropertyType::String,
        "int" => PropertyType::Int,
        "float" => PropertyType::Float,
        "bool" => PropertyType::Bool,
        "color" => PropertyType::Color,
        "file" => PropertyType::File,
        _ => panic!("Unsupported propertytype: {}", propertytype),
    }
}

fn staggerindex_from_string(staggerindex: &String) -> StaggerIndex {
    match staggerindex.to_lowercase().as_ref() {
        "even" => StaggerIndex::Even,
        "odd" => StaggerIndex::Odd,
        _ => panic!("Unsupported staggerindex: {}", staggerindex),
    }
}

fn staggeraxis_from_string(staggeraxis: &String) -> StaggerAxis {
    match staggeraxis.to_lowercase().as_ref() {
        "x" => StaggerAxis::X,
        "y" => StaggerAxis::Y,
        _ => panic!("Unsupported staggeraxis: {}", staggeraxis),
    }
}

fn renderorder_from_string(renderorder: &String) -> Renderorder {
    match renderorder.to_lowercase().as_ref() {
        "right-down" => Renderorder::RightDown,
        "right-up" => Renderorder::RightUp,
        "left-down" => Renderorder::LeftDown,
        "left-up" => Renderorder::LeftUp,
        _ => panic!("Unsupported renderorder: {}", renderorder),
    }
}

fn orientation_from_string(orientation: &String) -> Orientation {
    match orientation.to_lowercase().as_ref() {
        "orthogonal" => Orientation::Orthogonal,
        "isometric" => Orientation::Isometric,
        "staggered" => Orientation::Staggered,
        "hexagonal" => Orientation::Hexagonal,
        _ => panic!("Unsupported orientation: {}", orientation),
    }
}

fn draworder_from_string(draworder: &String) -> DrawOrder {
    match draworder.to_lowercase().as_ref() {
        "index" => DrawOrder::Index,
        "topdown" => DrawOrder::TopDown,
        _ => panic!("Unsupported draworder: {}", draworder),
    }
}

fn read_property(e: &BytesStart) -> Property {

    let kv = extract_attributes(e);

    Property {
        name: kv.get("name").map(|s| s.to_string()).unwrap(),
        value: kv.get("value").map(|s| s.to_string()).unwrap(),
        prop_type: kv.get("type").map(|s| propertytype_from_string(s)).unwrap_or(PropertyType::String),
    }
}

fn read_tileoffset(e: &BytesStart) -> TileOffset {

    let kv = extract_attributes(e);

    TileOffset {
        x: kv.get("x").unwrap().parse::<i64>().unwrap(),
        y: kv.get("y").unwrap().parse::<i64>().unwrap(),
    }
}

fn read_properties(reader: &mut Reader<&[u8]>) -> Vec<Property> {

    let mut properties: Vec<Property> = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    _ => exhaust(reader),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"property" => properties.push(read_property(e)),
                    _           => exhaust(reader),
                }
            },
            Ok(Event::End(ref e)) => {match e.name() {
                b"properties"   => break,
                _               => (),
            }},
            Err(e)          => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof)  => break,
            _               => (),
        }
    }
    buf.clear();

    properties
}

fn read_image(e: &BytesStart) -> Image {

    let kv = extract_attributes(e);

    Image {
        format: kv.get("format").map(|s| s.to_string()),
        source: kv.get("source").unwrap().to_string(),
        trans: kv.get("trans").map(|s| s.to_string()),
        width: kv.get("width").unwrap().parse::<i32>().unwrap(),
        height: kv.get("height").map(|s| s.parse::<i32>().unwrap()),
    }
}


fn read_object(e: &BytesStart, reader: &mut Reader<&[u8]>, is_empty: bool) -> Object{

    let mut properties: Option<Vec<Property>> = None;

    if !is_empty {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"properties" => properties = Some(read_properties(reader)),
                        _ => exhaust(reader),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        _ => exhaust(reader),
                    }
                },
                Ok(Event::End(ref e)) => {match e.name() {
                    b"object" => break,
                    _ => (),
                }},
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
        }
    }

    let kv = extract_attributes(e);

    let gid = kv.get("gid").map(|gid| gid.to_string().parse::<i64>().unwrap());
    Object {
        x:kv.get("x").unwrap().parse::<f32>().unwrap(),
        y:kv.get("y").unwrap().parse::<f32>().unwrap(),
        width: kv.get("width").unwrap_or(&"0".to_string()).parse::<f32>().unwrap(),
        height: kv.get("height").unwrap_or(&"0".to_string()).parse::<f32>().unwrap(),
        id: kv.get("id").unwrap().parse::<i64>().unwrap(),
        name: kv.get("name").map(|s| s.to_string()),
        visible: kv.get("visible").unwrap_or(&"true".to_string()).parse::<bool>().unwrap(),
        object_type: kv.get("type").map(|s| s.to_string()),
        template: kv.get("template").map(|s| s.to_string()),
        rotation: kv.get("rotation").unwrap_or(&"0".to_string()).parse::<f32>().unwrap(),
        gid: gid.map(|gid| gid & CLEAR_MASK),
        properties: properties,
        flipped_horizontally: gid.map(|gid| gid & FLIPPED_HORIZONTALLY_FLAG > 0).unwrap_or(false),
        flipped_diagonally: gid.map(|gid| gid & FLIPPED_DIAGONALLY_FLAG > 0).unwrap_or(false),
        flipped_vertically: gid.map(|gid| gid & FLIPPED_VERTICALLY_FLAG > 0).unwrap_or(false),
    }
}

fn read_objectgroup(e: &BytesStart, reader: &mut Reader<&[u8]>) -> Objectgroup{

    let mut objects: Vec<Object> = Vec::new();
    let mut properties: Option<Vec<Property>> = None;

    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"object" => {
                        objects.push(read_object(e, reader, true))
                    },
                    _ => exhaust(reader),
                };
            },
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"object"       => objects.push(read_object(e, reader, false)) ,
                    b"properties"   => properties = Some(read_properties(reader)),
                    _               => exhaust(reader),
                }
            },
            Ok(Event::End(ref e)) => {match e.name() {
                b"objectgroup"  => break,
                _               => (),
            }},
            Ok(Event::Eof)  => break,
            Err(e)          => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _               => (),
        }
    }

    let kv = extract_attributes(e);
    Objectgroup {
        x:kv.get("x").unwrap_or(&"0".to_string()).parse::<f32>().unwrap(),
        y:kv.get("y").unwrap_or(&"0".to_string()).parse::<f32>().unwrap(),
        offsetx:kv.get("offsetx").unwrap_or(&"0".to_string()).parse::<i32>().unwrap(),
        offsety:kv.get("offsety").unwrap_or(&"0".to_string()).parse::<i32>().unwrap(),
        color: kv.get("color").unwrap_or(&String::new()).to_string(),
        draworder: kv.get("draworder").map(|s| draworder_from_string(s)).unwrap_or(DrawOrder::TopDown),
        id: kv.get("id").unwrap_or(&"0".to_string()).parse::<i64>().unwrap(),
        name: kv.get("name").unwrap().to_string(),
        opacity: kv.get("opacity").unwrap_or(&"1.0".to_string()).parse::<f32>().unwrap(),
        visible: kv.get("visible").map(|s| s == "0").unwrap_or(true),
        objects: objects,
        properties: properties,
    }
}

fn read_tile(e: &BytesStart, reader: &mut Reader<&[u8]>) -> Tile {

    let mut buf = Vec::new();

    let mut image: Option<Image> = None;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    _ => exhaust(reader),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"image" => image = Some(read_image(e)),
                    _ => exhaust(reader),
                }
                break;
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    let kv = extract_attributes(e);
    Tile {
        id: kv.get("id").unwrap().parse::<i64>().unwrap(),
        terrain: kv.get("terrain").map(|s| s.to_string()),
        probability: kv.get("probability").unwrap_or(&"1".to_string()).parse::<u32>().unwrap(),
        tile_type: kv.get("tile_type").map(|s| s.to_string()),
        image: image,
    }
}

fn read_tileset(e: &BytesStart, reader: &mut Reader<&[u8]>) -> Tileset{

    let mut buf = Vec::new();
    let mut tiles = Vec::new();
    let mut tile_offset: Option<TileOffset> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"tile" => tiles.push(read_tile(e, reader)),
                    _       => exhaust(reader),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"tileoffset" => tile_offset = Some(read_tileoffset(e)),
                    _ => exhaust(reader),
                }
            },
            Ok(Event::End(ref e)) => {match e.name() {
                b"tileset" => break,
                _ => (),
            }},
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    let kv = extract_attributes(e);
    Tileset {
        firstgid: kv.get("firstgid").unwrap().parse::<i64>().unwrap(),
        name: kv.get("name").unwrap().to_string(),
        tilewidth: kv.get("tilewidth").unwrap().parse::<i32>().unwrap(),
        tileheight: kv.get("tileheight").unwrap().parse::<i32>().unwrap(),
        tilecount: kv.get("tilecount").unwrap().parse::<i32>().unwrap(),
        columns: kv.get("columns").unwrap().parse::<i32>().unwrap(),
        margin: 0,
        source: kv.get("name").map(|s| s.to_string()),
        spacing: 0,
        tiles: tiles,
        tileoffset: tile_offset,
    }
}

fn read_map(e: &BytesStart, reader: &mut Reader<&[u8]>) -> Map{


    let mut buf = Vec::new();

    let mut tileset: Option<Tileset> = None;
    let mut objectgroups: Vec<Objectgroup> = Vec::new();
    let mut properties: Option<Vec<Property>> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"objectgroup"  => objectgroups.push(read_objectgroup(e, reader)),
                    b"tileset"      => tileset = Some(read_tileset(e, reader)),
                    b"properties"   => properties = Some(read_properties(reader)),
                    _               => exhaust(reader),
                }
            },
            Err(e)          => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof)  => break,
            _               => (),
        }
    }

    let kv = extract_attributes(e);
    Map {
        version: kv.get("version").unwrap().to_string(),
        orientation: orientation_from_string(kv.get("orientation").unwrap()),
        renderorder: kv.get("renderorder").map(|s| renderorder_from_string(s)).unwrap_or(Renderorder::RightDown),
        width: kv.get("width").unwrap().parse::<i32>().unwrap(),
        height: kv.get("height").unwrap().parse::<i32>().unwrap(),
        tilewidth: kv.get("tilewidth").unwrap().parse::<i32>().unwrap(),
        tileheight: kv.get("tileheight").unwrap().parse::<i32>().unwrap(),
        backgroundcolor: kv.get("backgroundcolor").unwrap().to_string(),
        nextobjectid: kv.get("nextobjectid").unwrap().parse::<i32>().unwrap(),
        staggeraxis: kv.get("staggeraxis").map(|s| staggeraxis_from_string(s)).unwrap_or(StaggerAxis::X),
        staggerindex: kv.get("staggerindex").map(|s| staggerindex_from_string(s)).unwrap_or(StaggerIndex::Even),
        objectgroups: objectgroups,
        tileset: tileset.unwrap(),
        properties: properties
    }
}

fn extract_attributes(e: &BytesStart) -> HashMap<String, String> {
    e.attributes().map(|a| {
        let attribute = a.unwrap();
        let key = attribute.key;
        let mut value = attribute.value;
        (str::from_utf8(key).unwrap().to_string().to_lowercase(), str::from_utf8(value.to_mut()).unwrap().to_string())
    }).collect::<HashMap<String, String>>()
}

fn exhaust(reader: &mut Reader<&[u8]>) {
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref _e)) => {exhaust(reader)},
            Ok(Event::Empty(ref _e)) => {exhaust(reader); break},
            Ok(Event::End(ref _e)) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs;
    use super::*;

    #[test]
    fn test_read_sandbox() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        d.push("sandbox.tmx");

        let contents = fs::read_to_string(d.as_os_str())
            .expect("Something went wrong reading the file");

        let map = Map::new(contents.as_str());

        test_map(&map);

        test_tileset(&map.tileset);

        test_objectgroups(map.objectgroups);
    }

    fn test_map(map: &Map) {
        assert!(map.version == "1.0");
        assert!(map.orientation == Orientation::Orthogonal);
        assert!(map.renderorder == Renderorder::RightDown);
        assert!(map.width == 79);
        assert!(map.height == 45);
        assert!(map.tilewidth == 32);
        assert!(map.tileheight == 32);
        assert!(map.backgroundcolor == "#27b99a");
        assert!(map.nextobjectid == 203);
        assert!(map.objectgroups.len() == 9);
    }

    fn test_tileset(tileset: &Tileset) {
        assert!(tileset.firstgid == 1);
        assert!(tileset.name == "objs");
        assert!(tileset.tilewidth == 384);
        assert!(tileset.tileheight == 332);
        assert!(tileset.tilecount == 62);
        assert!(tileset.columns == 0);
        assert!(tileset.tiles.len() as i32 == tileset.tilecount);

        let tile: &Tile = tileset.tiles.get(0).unwrap();

        assert!(tile.probability == 1);
        assert!(tile.id == 0);

        let image:&Image = tile.image.as_ref().unwrap();
        assert!(image.source == "alter.png");
        assert!(image.width == 160);
        assert!(image.height.unwrap() == 192);
    }

    fn test_objectgroups(objectgroups: Vec<Objectgroup>) {

        assert!(objectgroups.get(0).unwrap().name == "parallax");
        assert!(objectgroups.get(1).unwrap().name == "background");
        assert!(objectgroups.get(2).unwrap().name == "ground");
        assert!(objectgroups.get(3).unwrap().name == "castle");
        assert!(objectgroups.get(4).unwrap().name == "castledeco");
        assert!(objectgroups.get(5).unwrap().name == "shading");
        assert!(objectgroups.get(6).unwrap().name == "game");
        assert!(objectgroups.get(7).unwrap().name == "above");
        assert!(objectgroups.get(8).unwrap().name == "bounds");

        let objectgroup = objectgroups.get(2).unwrap();

        let object:&Object = objectgroup.objects.get(0).as_ref().unwrap();

        assert!(object.id == 2);
        assert!(object.gid.unwrap() == 31);
        assert!(object.x == 0.0);
        assert!(object.y == 1087.0);
        assert!(object.width == 256.0);
        assert!(object.height == 96.0);

        let properties: &Vec<Property> = object.properties.as_ref().unwrap();
        assert!(properties.len() == 2);

        let property:&Property = properties.get(1).unwrap();

        assert!(property.name == "friction");
        assert!(property.prop_type == PropertyType::Float);
        assert!(property.value == "1");

        let objectgroup_parallax = objectgroups.get(0).unwrap();
        let object_flippedcloud:&Object = objectgroup_parallax.objects.get(2).as_ref().unwrap();

        assert!(object_flippedcloud.id == 91);
        assert!(object_flippedcloud.gid.unwrap() == 7);
        assert!(object_flippedcloud.x == 373.939);
        assert!(object_flippedcloud.y == 627.121);
        assert!(object_flippedcloud.width == 384.0);
        assert!(!object_flippedcloud.flipped_vertically);
        assert!(!object_flippedcloud.flipped_diagonally);
        assert!(object_flippedcloud.flipped_horizontally);
    }

    #[test]
    fn test_read_desert() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        d.push("desert.tmx");

        let contents = fs::read_to_string(d.as_os_str())
            .expect("Something went wrong reading the file");

        let map = Map::new(contents.as_str());

        assert!(map.version == "1.0");
        assert!(map.orientation == Orientation::Orthogonal);
        assert!(map.renderorder == Renderorder::RightDown);
        assert!(map.width == 40);
        assert!(map.height == 40);
        assert!(map.tilewidth == 32);
        assert!(map.tileheight == 32);
    }
}
