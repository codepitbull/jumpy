# TMX Reader
This library provides a reader for [TMX](https://doc.mapeditor.org/en/stable/reference/tmx-map-format/) files.

These files can be created using the great [Tiled](https://www.mapeditor.org/) editor.

# What is supported

- Tilesets
- Objectgroups
- Properties

# What is missing

- Layers
- imagelayer
- Grid (and all other isometric stuff)
- embedded data
- terrain
- animation
- wangsets
- ...

So there is still a ton of thiungs to do.

# Usage

```
use tmx_reader::Map;
use std::path::PathBuf;
use std::fs;

let mut d = PathBuf::from(env!("sandbox.tmx"));

let contents = fs::read_to_string(d.as_os_str())
    .expect("Something went wrong reading the file");

let map = Map::new(contents.as_str());

```

# Aim
I have no idea if this will ever be a useful crate :)

I just use this for my own little game experiments.