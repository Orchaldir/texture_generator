use crate::rendering::wall::WallStyle;
use crate::tilemap::border::Border;
use crate::tilemap::tilemap2d::Tilemap2d;
use crate::tilemap::Side;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use texture_generation::utils::resource::ResourceManager;

pub fn calculate_node_style<'a, T>(
    wall_styles: &'a ResourceManager<WallStyle<T>>,
    tilemap: &'a Tilemap2d,
    index: usize,
) -> Option<&'a T> {
    let sides_per_style = calculate_sides_per_style(tilemap, index);
    let top_styles = get_top_styles(sides_per_style);

    select_best_node_style(wall_styles, top_styles)
}

fn calculate_sides_per_style(tilemap: &Tilemap2d, index: usize) -> HashMap<usize, Vec<Side>> {
    let mut wall_styles = HashMap::new();

    for side in Side::iterator() {
        if let Border::Wall(id) = tilemap.get_border_at_node(index, *side) {
            match wall_styles.entry(id) {
                Entry::Vacant(e) => {
                    e.insert(vec![*side]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(*side);
                }
            }
        }
    }

    wall_styles
}

fn get_top_styles(input: HashMap<usize, Vec<Side>>) -> Vec<(usize, Vec<Side>)> {
    let mut max_count = 0;
    let mut top_styles = Vec::new();

    for entry in input {
        let count = entry.1.len();

        if count > max_count {
            max_count = count;
            top_styles.clear();
            top_styles.push(entry);
        } else if count > 0 && count == max_count {
            top_styles.push(entry);
        }
    }

    top_styles
}

fn select_best_node_style<T>(
    wall_styles: &ResourceManager<WallStyle<T>>,
    top_styles: Vec<(usize, Vec<Side>)>,
) -> Option<&T> {
    if top_styles.len() == 1 {
        let top_style = &top_styles[0];
        let side_count = top_style.1.len();

        if side_count == 2 && is_straight(top_style) {
            return get_node_style(wall_styles, top_style.0);
        }

        return get_corner_style(wall_styles, top_style.0);
    } else if top_styles.len() > 1 {
        let side_count = top_styles[0].1.len();

        if side_count == 2 {
            let style0 = &top_styles[0];
            let style1 = &top_styles[1];

            if is_straight(style0) {
                return get_corner_style(wall_styles, style0.0);
            } else if is_straight(style1) {
                return get_corner_style(wall_styles, style1.0);
            }
        }

        let best_id = select_best_wall_style(wall_styles, top_styles);
        return get_corner_style(wall_styles, best_id);
    }

    None
}

fn select_best_wall_style<T>(
    wall_styles: &ResourceManager<WallStyle<T>>,
    top_styles: Vec<(usize, Vec<Side>)>,
) -> usize {
    let mut best_id = top_styles[0].0;
    let mut best_wall_style = wall_styles.get(best_id).unwrap();

    for (id, _sides) in top_styles {
        let wall_style = wall_styles.get(id).unwrap();

        if wall_style.is_greater(best_wall_style) {
            best_id = id;
            best_wall_style = wall_style;
        }
    }

    best_id
}

fn get_corner_style<T>(wall_styles: &ResourceManager<WallStyle<T>>, index: usize) -> Option<&T> {
    wall_styles
        .get(index)
        .map(|wall_style| wall_style.get_corner_style())
}

fn get_node_style<T>(wall_styles: &ResourceManager<WallStyle<T>>, index: usize) -> Option<&T> {
    wall_styles
        .get(index)
        .and_then(|wall_style| Option::from(wall_style.get_node_style()))
}

fn is_straight(entry: &(usize, Vec<Side>)) -> bool {
    let side0 = entry.1[0];
    let side1 = entry.1[1];

    side0.is_straight(side1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::wall::EdgeStyle;
    use crate::tilemap::tile::Tile;
    use crate::tilemap::Side::{Left, Top};
    use texture_generation::math::size::Size;

    // wall styles
    const LOW: usize = 0;
    const LOW_WITH_NODE: usize = 1;
    const HIGH: usize = 2;
    const HIGH_WITH_NODE: usize = 3;

    // Nodes styles
    const LOW_CORNER: usize = 10;
    const LOW_NODE: usize = 11;
    const HIGH_CORNER: usize = 12;
    const HIGH_NODE: usize = 13;

    #[test]
    fn test_single_horizontal_wall() {
        let wall_styles = crate_wall_styles();
        let size = Size::new(1, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Top, Border::Wall(LOW));

        assert_style(&wall_styles, &tilemap, 0, Some(&LOW_CORNER));
        assert_style(&wall_styles, &tilemap, 1, Some(&LOW_CORNER));
        assert_style(&wall_styles, &tilemap, 2, None);
        assert_style(&wall_styles, &tilemap, 3, None);
    }

    #[test]
    fn test_long_vertical_wall() {
        let wall_styles = crate_wall_styles();
        let size = Size::new(1, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Left, Border::Wall(HIGH));
        tilemap.set_border(1, Left, Border::Wall(HIGH));

        println!("horizontal={:?}", tilemap.get_horizontal_borders());
        println!("vertical={:?}", tilemap.get_vertical_borders());

        assert_style(&wall_styles, &tilemap, 0, Some(&HIGH_CORNER));
        assert_style(&wall_styles, &tilemap, 1, None);
        assert_style(&wall_styles, &tilemap, 2, None);
        assert_style(&wall_styles, &tilemap, 3, None);
        assert_style(&wall_styles, &tilemap, 4, Some(&HIGH_CORNER));
        assert_style(&wall_styles, &tilemap, 5, None);
    }

    fn crate_wall_styles() -> ResourceManager<WallStyle<usize>> {
        let low_style = crate_wall_style(10, None, LOW_CORNER);
        let low_style_with_nodes = crate_wall_style(10, Some(LOW_NODE), LOW_CORNER);
        let high_style = crate_wall_style(20, None, HIGH_CORNER);
        let high_style_with_nodes = crate_wall_style(20, Some(HIGH_NODE), HIGH_CORNER);

        ResourceManager::new(vec![
            low_style,
            low_style_with_nodes,
            high_style,
            high_style_with_nodes,
        ])
    }

    fn crate_wall_style(
        wall_thickness: u32,
        node_style: Option<usize>,
        corner_style: usize,
    ) -> WallStyle<usize> {
        let edge_style = EdgeStyle::Mock(wall_thickness);
        WallStyle::new("test", edge_style, node_style, corner_style)
    }

    fn assert_style(
        wall_styles: &ResourceManager<WallStyle<usize>>,
        tilemap: &Tilemap2d,
        index: usize,
        style: Option<&usize>,
    ) {
        assert_eq!(calculate_node_style(wall_styles, tilemap, index), style);
    }
}
