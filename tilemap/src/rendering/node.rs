use crate::rendering::style::node::NodeStyle;
use crate::rendering::style::wall::WallStyle;
use crate::tilemap::node::get_nodes_size;
use crate::tilemap::tilemap2d::Tilemap2d;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use texture_generation::math::side::Side;
use texture_generation::utils::resource::ResourceManager;

#[derive(Debug, PartialEq)]
pub enum NodeStatus<'a> {
    Nothing,
    RenderNode(&'a NodeStyle),
    RenderEdge(i32, Side),
}

impl<'a> NodeStatus<'a> {
    pub fn calculate_half(&self, side: Side) -> i32 {
        match self {
            NodeStatus::Nothing => 0,
            NodeStatus::RenderNode(style) => style.get_half(),
            NodeStatus::RenderEdge(half, dominant_side) => {
                if side == *dominant_side {
                    -*half
                } else {
                    *half
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum InternalNode {
    Nothing,
    RenderNode(usize),
    RenderEdge(usize, Vec<Side>),
}

pub fn calculate_node_styles<'a>(
    node_styles: &'a ResourceManager<NodeStyle>,
    wall_styles: &'a ResourceManager<WallStyle>,
    tilemap: &'a Tilemap2d,
) -> Vec<NodeStatus<'a>> {
    calculate_node_style_ids(wall_styles, tilemap)
        .into_iter()
        .enumerate()
        .map(|(i, node)| match node {
            InternalNode::Nothing => NodeStatus::Nothing,
            InternalNode::RenderNode(id) => NodeStatus::RenderNode(node_styles.get(id)),
            InternalNode::RenderEdge(id, sides) => {
                let thickness = wall_styles.get(id).get_edge_style().get_thickness() as i32;
                if let Some(best_side) = sides
                    .iter()
                    .find(|s| tilemap.get_border_at_node(i, **s).is_wall())
                {
                    NodeStatus::RenderEdge(thickness / 2, *best_side)
                } else {
                    NodeStatus::Nothing
                }
            }
        })
        .collect()
}

fn calculate_node_style_ids(
    wall_styles: &ResourceManager<WallStyle>,
    tilemap: &Tilemap2d,
) -> Vec<InternalNode> {
    let size = get_nodes_size(tilemap.get_size());
    let mut node_styles = Vec::with_capacity(size.len());
    let mut index = 0;

    for _y in 0..size.height() {
        for _x in 0..size.width() {
            node_styles.push(calculate_node_style(wall_styles, tilemap, index));
            index += 1;
        }
    }

    node_styles
}

fn calculate_node_style(
    wall_styles: &ResourceManager<WallStyle>,
    tilemap: &Tilemap2d,
    index: usize,
) -> InternalNode {
    let sides_per_style = calculate_sides_per_style(tilemap, index);
    let is_corner = sides_per_style.len() > 1;
    let top_styles = get_top_styles(sides_per_style);

    select_best_node_style(wall_styles, top_styles, is_corner)
}

fn calculate_sides_per_style(tilemap: &Tilemap2d, index: usize) -> HashMap<usize, Vec<Side>> {
    let mut wall_styles = HashMap::new();

    for side in Side::iterator() {
        let wall_style = tilemap.get_border_at_node(index, *side).get_wall_style();

        if let Some(id) = wall_style {
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

fn select_best_node_style(
    wall_styles: &ResourceManager<WallStyle>,
    top_styles: Vec<(usize, Vec<Side>)>,
    is_corner: bool,
) -> InternalNode {
    match top_styles.len() {
        1 => {
            let top_style = &top_styles[0];
            let side_count = top_style.1.len();

            if !is_corner && side_count == 2 && is_straight(top_style) {
                return get_node_style(wall_styles, top_style.0);
            }

            get_corner_style(wall_styles, top_style)
        }
        n if n > 1 => {
            let side_count = top_styles[0].1.len();

            if side_count == 2 {
                let style0 = &top_styles[0];
                let style1 = &top_styles[1];
                let is_straight0 = is_straight(style0);
                let is_straight1 = is_straight(style1);

                if is_straight0 && !is_straight1 {
                    return get_corner_style(wall_styles, style0);
                } else if is_straight1 && !is_straight0 {
                    return get_corner_style(wall_styles, style1);
                }
            }

            let best_style = select_best_wall_style(wall_styles, top_styles);
            get_corner_style(wall_styles, &best_style)
        }
        _ => InternalNode::Nothing,
    }
}

fn select_best_wall_style(
    wall_styles: &ResourceManager<WallStyle>,
    top_styles: Vec<(usize, Vec<Side>)>,
) -> (usize, Vec<Side>) {
    let mut best = &top_styles[0];
    let mut best_wall_style = wall_styles.get(best.0);

    for entry in &top_styles {
        let wall_style = wall_styles.get(entry.0);

        if wall_style.is_greater(best_wall_style) {
            best = &entry;
            best_wall_style = wall_style;
        }
    }

    best.clone()
}

fn get_corner_style(
    wall_styles: &ResourceManager<WallStyle>,
    top_style: &(usize, Vec<Side>),
) -> InternalNode {
    match wall_styles.get(top_style.0).get_corner_style() {
        None => InternalNode::RenderEdge(top_style.0, top_style.1.clone()),
        Some(id) => InternalNode::RenderNode(id),
    }
}

fn get_node_style(wall_styles: &ResourceManager<WallStyle>, index: usize) -> InternalNode {
    match wall_styles.get(index).get_node_style() {
        None => InternalNode::Nothing,
        Some(id) => InternalNode::RenderNode(id),
    }
}

fn is_straight(entry: &(usize, Vec<Side>)) -> bool {
    let side0 = entry.1[0];
    let side1 = entry.1[1];

    side0.is_straight(side1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::node::InternalNode::{Nothing, RenderEdge, RenderNode};
    use crate::rendering::style::edge::EdgeStyle;
    use crate::tilemap::border::Border;
    use crate::tilemap::tile::Tile;
    use texture_generation::math::side::Side::{Bottom, Left, Right, Top};
    use texture_generation::math::size::Size;

    // wall styles
    const LOW: usize = 0;
    const LOW_WITH_NODE: usize = 1;
    const HIGH: usize = 2;
    const HIGH_WITH_NODE: usize = 3;
    const WITHOUT_CORNERS: usize = 4;

    // Nodes styles
    const LOW_CORNER: usize = 10;
    const LOW_NODE: usize = 11;
    const HIGH_CORNER: usize = 12;
    const HIGH_NODE: usize = 13;
    const LOW_CORNER2: usize = 14;
    const HIGH_CORNER2: usize = 15;

    #[test]
    fn test_calculate_half_of_nothing() {
        for side in Side::iterator() {
            assert_eq!(NodeStatus::Nothing.calculate_half(*side), 0);
        }
    }

    #[test]
    fn test_calculate_half_of_node() {
        let style = NodeStyle::default_with_size(24);
        let node = NodeStatus::RenderNode(&style);

        for side in Side::iterator() {
            assert_eq!(node.calculate_half(*side), 12);
        }
    }

    #[test]
    fn test_calculate_half_of_edge() {
        let node = NodeStatus::RenderEdge(33, Right);

        assert_eq!(node.calculate_half(Top), 33);
        assert_eq!(node.calculate_half(Left), 33);
        assert_eq!(node.calculate_half(Bottom), 33);
        assert_eq!(node.calculate_half(Right), -33);
    }

    #[test]
    fn test_single_horizontal_wall() {
        let wall_styles = create_wall_styles();
        let size = Size::new(1, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Top, Border::Wall(LOW));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                RenderNode(LOW_CORNER2), RenderNode(LOW_CORNER2),
                Nothing, Nothing
            ]
        );
    }

    #[test]
    fn test_long_vertical_wall() {
        let wall_styles = create_wall_styles();
        let size = Size::new(1, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Left, Border::Wall(HIGH));
        tilemap.set_border(1, Left, Border::Wall(HIGH));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                RenderNode(HIGH_CORNER2), Nothing,
                Nothing, Nothing,
                RenderNode(HIGH_CORNER2), Nothing
            ]
        );
    }

    #[test]
    fn test_long_vertical_wall_with_door() {
        let wall_styles = create_wall_styles();
        let size = Size::new(1, 3);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Left, Border::Wall(HIGH));
        tilemap.set_border(1, Left, Border::new_door(HIGH, 0, false));
        tilemap.set_border(2, Left, Border::Wall(HIGH));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                RenderNode(HIGH_CORNER2), Nothing,
                Nothing, Nothing,
                Nothing, Nothing,
                RenderNode(HIGH_CORNER2), Nothing
            ]
        );
    }

    #[test]
    fn test_long_horizontal_wall_with_nodes() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(LOW_WITH_NODE));
        tilemap.set_border(1, Bottom, Border::Wall(LOW_WITH_NODE));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, Nothing, Nothing,
                RenderNode(LOW_CORNER), RenderNode(LOW_NODE), RenderNode(LOW_CORNER)
            ]
        );
    }

    #[test]
    fn test_corner() {
        let wall_styles = create_wall_styles();
        let size = Size::new(1, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(0, Right, Border::Wall(HIGH_WITH_NODE));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, RenderNode(HIGH_CORNER),
                RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER)
            ]
        );
    }

    #[test]
    fn test_t_crossing() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Top, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(0, Right, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(1, Top, Border::Wall(HIGH_WITH_NODE));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER),
                Nothing, RenderNode(HIGH_CORNER), Nothing
            ]
        );
    }

    #[test]
    fn test_crossing() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(0, Right, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(1, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(2, Right, Border::Wall(HIGH_WITH_NODE));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, RenderNode(HIGH_CORNER), Nothing,
                RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER),
                Nothing, RenderNode(HIGH_CORNER), Nothing
            ]
        );
    }

    /// Greater wall style gets the node.
    #[test]
    fn test_two_different_styles_straight() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(1, Bottom, Border::Wall(LOW_WITH_NODE));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, Nothing, Nothing,
                RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER), RenderNode(LOW_CORNER),
            ]
        );
    }

    /// Greater wall style gets the node.
    #[test]
    fn test_two_different_styles_corner() {
        let wall_styles = create_wall_styles();
        let size = Size::new(1, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Left, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(0, Top, Border::Wall(LOW_WITH_NODE));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                RenderNode(HIGH_CORNER), RenderNode(LOW_CORNER),
                RenderNode(HIGH_CORNER), Nothing
            ]
        );
    }

    #[test]
    fn test_t_crossing_with_dominant_style_straight() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Top, Border::Wall(LOW));
        tilemap.set_border(0, Right, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(1, Top, Border::Wall(LOW));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                RenderNode(LOW_CORNER2), RenderNode(LOW_CORNER2), RenderNode(LOW_CORNER2),
                Nothing, RenderNode(HIGH_CORNER), Nothing
            ]
        );
    }

    #[test]
    fn test_t_crossing_with_dominant_style_corner() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Top, Border::Wall(LOW_WITH_NODE));
        tilemap.set_border(0, Right, Border::Wall(LOW_WITH_NODE));
        tilemap.set_border(1, Top, Border::Wall(HIGH));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                RenderNode(LOW_CORNER), RenderNode(LOW_CORNER), RenderNode(HIGH_CORNER2),
                Nothing, RenderNode(LOW_CORNER), Nothing
            ]
        );
    }

    #[test]
    fn test_crossing_with_dominant_style_straight() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(1, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(0, Right, Border::Wall(LOW_WITH_NODE));
        tilemap.set_border(2, Right, Border::Wall(LOW_WITH_NODE));

        println!("horizontal={:?}", tilemap.get_horizontal_borders());
        println!("vertical={:?}", tilemap.get_vertical_borders());

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, RenderNode(LOW_CORNER), Nothing,
                RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER),
                Nothing, RenderNode(LOW_CORNER), Nothing
            ]
        );
    }

    #[test]
    fn test_crossing_with_dominant_style_corner() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(0, Right, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(1, Bottom, Border::Wall(LOW_WITH_NODE));
        tilemap.set_border(2, Right, Border::Wall(LOW_WITH_NODE));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, RenderNode(HIGH_CORNER), Nothing,
                RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER), RenderNode(LOW_CORNER),
                Nothing, RenderNode(LOW_CORNER), Nothing
            ]
        );
    }

    #[test]
    fn test_crossing_with_4_styles() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(HIGH_WITH_NODE));
        tilemap.set_border(0, Right, Border::Wall(HIGH));
        tilemap.set_border(1, Bottom, Border::Wall(LOW_WITH_NODE));
        tilemap.set_border(2, Right, Border::Wall(LOW));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, RenderNode(HIGH_CORNER2), Nothing,
                RenderNode(HIGH_CORNER), RenderNode(HIGH_CORNER), RenderNode(LOW_CORNER),
                Nothing, RenderNode(LOW_CORNER2), Nothing
            ]
        );
    }

    #[test]
    fn test_without_corner() {
        let wall_styles = create_wall_styles();
        let size = Size::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_border(0, Bottom, Border::Wall(WITHOUT_CORNERS));
        tilemap.set_border(0, Right, Border::Wall(WITHOUT_CORNERS));

        #[rustfmt::skip]
        assert_eq!(
            calculate_node_style_ids(&wall_styles, &tilemap),
            vec![
                Nothing, RenderEdge(WITHOUT_CORNERS, vec![Bottom]), Nothing,
                RenderEdge(WITHOUT_CORNERS, vec![Right]), RenderEdge(WITHOUT_CORNERS, vec![Top, Left]), Nothing,
                Nothing, Nothing, Nothing
            ]
        );
    }

    fn create_wall_styles() -> ResourceManager<WallStyle> {
        let low_style = create_wall_style(10, None, LOW_CORNER2);
        let low_style_with_nodes = create_wall_style(15, Some(LOW_NODE), LOW_CORNER);
        let high_style = create_wall_style(20, None, HIGH_CORNER2);
        let high_style_with_nodes = create_wall_style(25, Some(HIGH_NODE), HIGH_CORNER);
        let without_corners = create_without_corners(25);

        ResourceManager::new(
            vec![
                low_style,
                low_style_with_nodes,
                high_style,
                high_style_with_nodes,
                without_corners,
            ],
            WallStyle::default(10),
        )
    }

    fn create_wall_style(
        wall_thickness: u32,
        node_style: Option<usize>,
        corner_style: usize,
    ) -> WallStyle {
        let edge_style = EdgeStyle::Mock(wall_thickness);
        WallStyle::new("test", edge_style, node_style, Some(corner_style))
    }

    fn create_without_corners(wall_thickness: u32) -> WallStyle {
        let edge_style = EdgeStyle::Mock(wall_thickness);
        WallStyle::new("test", edge_style, None, None)
    }
}
