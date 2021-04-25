use anyhow::{Context, Result};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::str::Split;
use texture_generation::math::size::Size;
use tilemap::tilemap::border::{get_horizontal_borders_size, get_vertical_borders_size, Border};
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;

pub fn load(path: &Path) -> Result<Tilemap2d> {
    info!("Load tilemap from '{:?}'", path);

    let file = File::open(path).context(format!("Unable to open '{:?}'", path))?;
    let mut reader = io::BufReader::new(file);

    let width = parse_u32(&mut reader, "width")?;
    let height = parse_u32(&mut reader, "height")?;
    let size = Size::new(width, height);
    let tiles = load_tiles(&mut reader, size)?;
    let horizontal_borders = load_borders(&mut reader, get_horizontal_borders_size(size))?;
    let vertical_borders = load_borders(&mut reader, get_vertical_borders_size(size))?;

    Tilemap2d::with_borders(size, tiles, horizontal_borders, vertical_borders)
        .ok_or_else(|| anyhow!("Could not create tilemap"))
}

fn load_tiles(reader: &mut BufReader<File>, size: Size) -> Result<Vec<Tile>> {
    let mut tiles = Vec::with_capacity(size.len());

    for y in 0..size.height() {
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

        if x > size.width() {
            return Err(anyhow!(
                "{}.row of tiles is too long with {} elements: '{}'",
                y + 1,
                x,
                line
            ));
        }
    }

    Ok(tiles)
}

fn load_borders(reader: &mut BufReader<File>, size: Size) -> Result<Vec<Border>> {
    let mut borders = Vec::with_capacity(size.len());

    for y in 0..size.height() {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .context(format!("Unable to read {}.row of borders", y + 1))?;

        let mut x = 0;

        for split in line.split(';') {
            let border = parse_border(split).context(format!(
                "Unable to read the {}.border of {}.row from '{}'",
                x + 1,
                y + 1,
                split,
            ))?;
            borders.push(border);
            x += 1;
        }

        if x > size.width() {
            return Err(anyhow!(
                "{}.row of borders is too long with {} elements: '{}'",
                y + 1,
                x,
                line
            ));
        }
    }

    Ok(borders)
}

pub fn save(tilemap: &Tilemap2d, path: &str) -> io::Result<()> {
    info!("Save tilemap to '{}'", path);
    let mut file = File::create(path)?;
    let size = tilemap.get_size();

    writeln!(&mut file, "width={}", size.width())?;
    writeln!(&mut file, "height={}", size.height())?;

    save_tiles(tilemap, &mut file)?;
    save_borders(
        tilemap.get_horizontal_borders(),
        get_horizontal_borders_size(size),
        &mut file,
    )?;
    save_borders(
        tilemap.get_vertical_borders(),
        get_vertical_borders_size(size),
        &mut file,
    )?;

    Ok(())
}

fn save_tiles(tilemap: &Tilemap2d, file: &mut File) -> io::Result<()> {
    let size = tilemap.get_size();
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

        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn save_borders(borders: &[Border], size: Size, file: &mut File) -> io::Result<()> {
    let capacity = (size.width() * 7) as usize;
    let mut index = 0;

    for _y in 0..size.height() {
        let mut line = String::with_capacity(capacity);

        for x in 0..size.width() {
            line.push_str(&format_border(&borders[index]));

            if x < size.width() - 1 {
                line.push(';');
            }

            index += 1;
        }

        writeln!(file, "{}", line)?;
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
        Some("F") => Ok(Tile::Floor(parse_usize("Tile Id", &mut parts)?)),
        Some("S") => Ok(Tile::Solid(parse_usize("Tile Id", &mut parts)?)),
        Some("E") => Ok(Tile::Empty),
        _ => Err(anyhow!("Unable to parse tile from '{}'", string)),
    }
}

fn parse_border(string: &str) -> Result<Border> {
    let mut parts = string.trim().split(',');

    match parts.next() {
        Some("E") => Ok(Border::Empty),
        Some("Wa") => Ok(Border::Wall(parse_usize("Wall Id", &mut parts)?)),
        Some("D") => Ok(Border::new_door(
            parse_usize("Wall Id", &mut parts)?,
            parse_usize("Door Id", &mut parts)?,
            parse_usize("Is Front", &mut parts)? == 1,
        )),
        Some("Wi") => Ok(Border::new_window(
            parse_usize("Wall Id", &mut parts)?,
            parse_usize("Window Id", &mut parts)?,
        )),
        _ => Err(anyhow!("Unable to parse border from '{}'", string)),
    }
}

fn parse_usize(name: &str, parts: &mut Split<char>) -> Result<usize> {
    if let Some(string) = parts.next() {
        string
            .parse()
            .context(format!("Unable to parse {} from '{}'", name, string))
    } else {
        Err(anyhow!("{} is missing", name))
    }
}

fn format_tile(tile: &Tile) -> String {
    match tile {
        Tile::Empty => "E  ".to_string(),
        Tile::Floor(id) => format!("F,{}", *id),
        Tile::Solid(id) => format!("S,{}", *id),
    }
}

fn format_border(border: &Border) -> String {
    match border {
        Border::Empty => "E      ".to_string(),
        Border::Wall(id) => format!("Wa,{}   ", *id),
        Border::Door {
            wall_id,
            door_id,
            is_front,
        } => format!("D,{},{},{}", *wall_id, *door_id, *is_front as usize),
        Border::Window { wall_id, window_id } => format!("Wi,{},{} ", *wall_id, *window_id),
    }
}
