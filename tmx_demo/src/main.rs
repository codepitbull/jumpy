// Draw an image to the screen
extern crate quicksilver;
extern crate image;
extern crate hex;
extern crate futures;

use std::collections::HashMap;
use tmx_reader::Map;
use regex::Regex;
use std::str;

use quicksilver::{
    Result,
    geom::{Shape, Vector, Rectangle, Transform},
    graphics::{Background::Img, Color, Image, View},
    input::{Key},
    lifecycle::{Settings, State, Window, run}
};

mod generated_mod {
    include!(concat!(env!("OUT_DIR"), "/static.rs"));
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

const FLIP_HORIZONTALLY:[[f32; 3]; 3] = [[1f32, 0f32, 1f32], [0f32, -1f32, 1f32], [0f32, 0f32, 1f32]];

const FLIP_VERTICALLY:[[f32; 3]; 3] = [[-1f32, 0f32, 1f32], [0f32, 1f32, 1f32], [0f32, 0f32, 1f32]];

struct TmxDemo {
    map: Map,
    tile_images: HashMap<i64, Image>,
    view: Rectangle,
    color: Color,
}

impl State for TmxDemo {

    fn new() -> Result<TmxDemo> {

        let resources: HashMap<&'static str, &'static[u8]> = generated_mod::static_content();


        let tmx_file = resources.get("sandbox.tmx").unwrap().to_vec();
        let tmx_content = str::from_utf8(&tmx_file);

        let map = Map::new(tmx_content.unwrap());

        let mut tile_images: HashMap<i64, Image> = HashMap::new();

        let firstgid = map.tileset.firstgid;

        map.tileset.tiles.iter().for_each(|tile| {
            let pos = firstgid + tile.id;
            let source:String = tile.image.as_ref().unwrap().source.clone();
            tile_images.insert(pos, Image::from_bytes(resources.get(&source[..]).unwrap()).unwrap());
        });

        let re = Regex::new(r"^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$").unwrap();

        let captures = re.captures(&map.backgroundcolor).unwrap();

        let r = hex::decode(&captures.get(1).unwrap().as_str()).unwrap().get(0).unwrap().to_owned() as f32 / 255.0;
        let g = hex::decode(&captures.get(2).unwrap().as_str()).unwrap().get(0).unwrap().to_owned() as f32 / 255.0;
        let b = hex::decode(&captures.get(3).unwrap().as_str()).unwrap().get(0).unwrap().to_owned() as f32 / 255.0;

        let color = Color { r: r, g: g, b: b, a: 1.0 };
        Ok(TmxDemo { map, tile_images, view: Rectangle::new_sized((800, 600)), color })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Left].is_down() {
            self.view = self.view.translate((-40, 0));
        }
        if window.keyboard()[Key::Right].is_down() {
            self.view = self.view.translate((40, 0));
        }
        if window.keyboard()[Key::Down].is_down() {
            self.view = self.view.translate((0, 40));
        }
        if window.keyboard()[Key::Up].is_down() {
            self.view = self.view.translate((0, -40));
        }
        window.set_view(View::new(self.view));
        Ok(())
    }


    fn draw(&mut self, window: &mut Window) -> Result<()> {

        window.clear(self.color)?;

        let map = &mut self.map;
        let images = &mut self.tile_images;

        let mut layers = Vec::new();
        map.objectgroups.iter().for_each(|objectgroup| {
            let mut vec = Vec::new();
            objectgroup.objects.iter().for_each(|object| {
                &object.gid.map(|tile_id| {
                    vec.push((tile_id, object))
                });
            });
            layers.push(vec);
        });

        let mut z = 0;
        layers.iter().for_each(|elems| {
            z = z + 1;
            elems.iter().for_each(|tileid_and_object| {
                images.get_mut(&tileid_and_object.0).map(|img| {
                    let object = tileid_and_object.1;
                    let rect: Rectangle = Rectangle::new((object.x, object.y - object.height), (object.width, object.height));
                    let mut trans: Transform = if object.rotation != 0.0  { Transform::rotate(object.rotation as f32) } else { Transform::IDENTITY };

                    if object.flipped_horizontally {
                        trans = trans * Transform::from_array(FLIP_VERTICALLY);
                    }
                    if object.flipped_vertically {
                        trans = trans * Transform::from_array(FLIP_HORIZONTALLY);
                    }
                    window.draw_ex(&rect, Img(img),  trans, z);
                });
            });
        });

        Ok(())
    }
}

fn main() {
    run::<TmxDemo>("tmxdemo", Vector::new(WIDTH, HEIGHT), Settings::default());
}
