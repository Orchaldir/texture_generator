use anyhow::{Context, Result};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::str::Split;
use texture_generation::math::size::Size;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;

pub fn load(filename: &str) -> Result<Tilemap2d> {
    info!("Load tilemap from '{}'", filename);

    let file = File::open(filename).context(format!("Unable to open '{}'", filename))?;
    let mut reader = io::BufReader::new(file);

    let width = parse_u32(&mut reader, "width")?;
    let height = parse_u32(&mut reader, "height")?;
    let size = Size::new(width, height);
    let mut tiles = Vec::with_capacity(size.len());

    for y in 0..height {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .context(format!("Unable to read {}.row of tiles", y + 1))?;

        let mut x = 0;

        for split in line.split(';') {
            let tile = parse_tile(split).context(format!(
                "Unable to read the {}.tile of {}.row from '{}'",
                x + 1,
                y + 1,
                split,
            ))?;
            tiles.push(tile);
            x += 1;
        }

        if x > width {
            return Err(anyhow!(
                "{}.row of tiles is too long with {} elements: '{}'",
                y + 1,
                x,
                line
            ));
        }
    }

    Tilemap2d::new(size, tiles).ok_or_else(|| anyhow!("Could not create tilemap"))
}

pub fn save(tilemap: &Tilemap2d, path: &str) -> io::Result<()> {
    info!("Save tilemap to '{}'", path);
    let mut file = File::create(path)?;
    let size = tilemap.get_size();

    writeln!(&mut file, "width={}", size.width())?;
    writeln!(&mut file, "height={}", size.height())?;

    let tiles = tilemap.get_tiles();
    let capacity = (size.width() * 4) as usize;
    let mut index = 0;

    for _y in 0..size.height() {
        let mut line = String::with_capacity(capacity);

        for x in 0..size.width() {
            line.push_str(&format_tile(&tiles[index]));

            if x < size.width() - 1 {
                line.push(';');
            }

            index += 1;
        }

        writeln!(&mut file, "{}", line)?;
    }

    Ok(())
}

fn parse_u32(reader: &mut BufReader<File>, value: &str) -> Result<u32> {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .context(format!("Unable to read line for '{}'", value))?;
    let line = line.trim();
    let prefix = format!("{}=", value);

    return if let Some(str) = line.strip_prefix(&prefix) {
        str.parse().context(format!("Unable to parse '{}'", value))
    } else {
        Err(anyhow!("Line doesn't start with {}", value))
    };
}

fn parse_tile(string: &str) -> Result<Tile> {
    let mut parts = string.trim().split(',');

    match parts.next() {
        Some("F") => Ok(Tile::Floor(parse_tile_id(&mut parts)?)),
        Some("S") => Ok(Tile::Solid(parse_tile_id(&mut parts)?)),
        Some("E") => Ok(Tile::Empty),
        Some(s) => Err(anyhow!("Unable to parse tilecdsaf from '{}'", s)),
        _ => Err(anyhow!("Unable to parse tile from '{}'", string)),
    }
}

fn parse_tile_id(parts: &mut Split<char>) -> Result<usize> {
    if let Some(string) = parts.next() {
        string
            .parse()
            .context(format!("Unable to parse tile id from '{}'", string))
    } else {
        Err(anyhow!("Tile id is missing"))
    }
}

fn format_tile(tile: &Tile) -> String {
    match tile {
        Tile::Empty => "E  ".to_string(),
        Tile::Floor(id) => format!("F,{}", *id),
        Tile::Solid(id) => format!("S,{}", *id),
    }
}
