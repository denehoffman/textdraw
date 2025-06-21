// use std::cmp::Ordering;
// use std::collections::{BinaryHeap, HashMap, HashSet};
// use std::fmt::Display;
//
// use pyo3::exceptions::PyRuntimeError;
// use pyo3::PyResult;
//
// use crate::enums::{ArrowStyle, Direction, LineStyle};
// use crate::{BoundingBox, Cell, Position, TextStyle};
//
// #[derive(Clone, Debug, Copy)]
// pub(crate) enum NodeCost {
//     Cost { cost: usize, group: Option<usize> },
//     Blocked,
// }
// impl From<Option<usize>> for NodeCost {
//     fn from(value: Option<usize>) -> Self {
//         match value {
//             Some(cost) => Self::Cost { cost, group: None },
//             None => Self::Blocked,
//         }
//     }
// }
// impl From<(Option<usize>, Option<usize>)> for NodeCost {
//     fn from(value: (Option<usize>, Option<usize>)) -> Self {
//         if let Some(pw) = value.0 {
//             NodeCost::Cost {
//                 cost: pw,
//                 group: value.1,
//             }
//         } else {
//             NodeCost::Blocked
//         }
//     }
// }
// impl Display for NodeCost {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             NodeCost::Cost {
//                 cost,
//                 group: Some(g),
//             } => {
//                 write!(f, "Cost({}, group={})", cost, g)
//             }
//             NodeCost::Cost { cost, group: None } => {
//                 write!(f, "Cost({})", cost)
//             }
//             NodeCost::Blocked => write!(f, "Blocked"),
//         }
//     }
// }
//
// #[derive(Clone, Debug)]
// struct Node {
//     neighbors: [bool; 4], // [up, right, down, left]
// }
//
// fn delta_to_direction(from: Position, to: Position) -> Option<Direction> {
//     let dx = to.x - from.x;
//     let dy = to.y - from.y;
//     Direction::from_delta(dx, dy)
// }
//
// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
// struct State {
//     cost: usize,
//     pos: Position,
//     dir: Option<Direction>,
// }
//
// impl Ord for State {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }
//
// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// fn heuristic(a: Position, b: Position) -> usize {
//     a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
// }
//
// fn astar(
//     start: Position,
//     goal: Position,
//     cost_map: &HashMap<Position, NodeCost>,
//     bend_penalty: usize,
//     bounding_box: BoundingBox,
//     group: Option<usize>,
// ) -> PyResult<Vec<Position>> {
//     let mut open = BinaryHeap::new();
//     let mut came_from = HashMap::new();
//     let mut g_score = HashMap::new();
//     let mut visited = HashSet::new();
//
//     g_score.insert(start, 0);
//     open.push(State {
//         cost: heuristic(start, goal),
//         pos: start,
//         dir: None,
//     });
//
//     while let Some(State { pos, dir, .. }) = open.pop() {
//         if pos == goal {
//             let mut path = vec![pos];
//             while let Some(&(p, _)) = came_from.get(path.last().unwrap()) {
//                 path.push(p);
//             }
//             path.reverse();
//             return Ok(path);
//         }
//
//         if visited.contains(&pos) {
//             continue;
//         }
//         visited.insert(pos);
//
//         for d in Direction::all() {
//             let (dx, dy) = d.offset().into();
//             let neighbor = Position {
//                 x: pos.x + dx,
//                 y: pos.y + dy,
//             };
//             if !bounding_box.contains(neighbor) {
//                 continue;
//             }
//
//             match cost_map.get(&neighbor).unwrap_or(&(Some(1).into())) {
//                 NodeCost::Blocked => continue,
//                 NodeCost::Cost {
//                     cost,
//                     group: node_group,
//                 } => {
//                     let bend = if let Some(prev_dir) = dir {
//                         if prev_dir != d {
//                             bend_penalty
//                         } else {
//                             0
//                         }
//                     } else {
//                         0
//                     };
//
//                     let mut tentative_g = g_score[&pos] + cost + bend;
//                     if let Some(ng) = node_group {
//                         if let Some(g) = group {
//                             if g == *ng {
//                                 tentative_g = g_score[&pos];
//                             }
//                         }
//                     }
//
//                     if tentative_g < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
//                         g_score.insert(neighbor, tentative_g);
//                         came_from.insert(neighbor, (pos, d));
//                         let f = tentative_g + heuristic(neighbor, goal);
//                         open.push(State {
//                             cost: f,
//                             pos: neighbor,
//                             dir: Some(d),
//                         });
//                     }
//                 }
//             }
//         }
//     }
//     Err(PyRuntimeError::new_err("Path could not be found"))
// }
//
// fn path_to_tile_chars(
//     path: &[Position],
//     extras: &HashSet<Position>,
//     style: TextStyle,
//     line_style: LineStyle,
//     start_style: TextStyle,
//     start_direction: Option<Direction>,
//     end_style: TextStyle,
//     end_direction: Option<Direction>,
//     path_weight: NodeCost,
// ) -> PyResult<HashMap<Position, Cell>> {
//     let mut prepend = None;
//     let mut append = None;
//
//     if path.len() >= 2 {
//         if let Some(dir) = start_direction.or_else(|| delta_to_direction(path[1], path[0])) {
//             let offset = dir.offset();
//             let virtual_start = path[0] + offset;
//             prepend = Some(virtual_start);
//         }
//
//         if let Some(dir) =
//             end_direction.or_else(|| delta_to_direction(path[path.len() - 2], path[path.len() - 1]))
//         {
//             let offset = dir.offset();
//             let virtual_end = path[path.len() - 1] + offset;
//             append = Some(virtual_end);
//         }
//     } else {
//         return Err(PyRuntimeError::new_err(
//             "Path must have at least two elements",
//         ));
//     }
//
//     let mut pos_set: HashSet<Position> = path.iter().copied().collect();
//     if let Some(pos) = prepend {
//         pos_set.insert(pos);
//     }
//     if let Some(pos) = append {
//         pos_set.insert(pos);
//     }
//     pos_set.extend(extras);
//     let mut tile_map: HashMap<Position, Node> = HashMap::new();
//     for pos in path {
//         let up_neighbor = pos_set.contains(&(*pos + Direction::Up.offset()));
//         let right_neighbor = pos_set.contains(&(*pos + Direction::Right.offset()));
//         let down_neighbor = pos_set.contains(&(*pos + Direction::Down.offset()));
//         let left_neighbor = pos_set.contains(&(*pos + Direction::Left.offset()));
//         tile_map.insert(
//             *pos,
//             Node {
//                 neighbors: [up_neighbor, right_neighbor, down_neighbor, left_neighbor],
//             },
//         );
//     }
//     let mut char_map = HashMap::new();
//     for (pos, tile) in tile_map {
//         let ch = line_style.get_char(&tile.neighbors);
//         let cell = Cell {
//             character: ch,
//             style: if pos == path[0] {
//                 start_style
//             } else if pos == path[path.len() - 1] {
//                 end_style
//             } else {
//                 style
//             },
//             weight: path_weight,
//         };
//         char_map.insert(pos, cell);
//     }
//
//     Ok(char_map)
// }
//
// pub(crate) fn make_path(
//     start: Position,
//     end: Position,
//     cost_map: &HashMap<Position, NodeCost>,
//     style: TextStyle,
//     start_arrow_style: Option<ArrowStyle>,
//     start_style: TextStyle,
//     start_direction: Option<Direction>,
//     end_arrow_style: Option<ArrowStyle>,
//     end_style: TextStyle,
//     end_direction: Option<Direction>,
//     line_style: LineStyle,
//     path_weight: NodeCost,
//     bounding_box: BoundingBox,
// ) -> PyResult<Vec<(Position, Cell)>> {
//     let mut extras = HashSet::new();
//     let path_group = if let NodeCost::Cost { cost: _, group } = path_weight {
//         if let Some(gr) = group {
//             for (pos, cost) in cost_map.iter() {
//                 if let NodeCost::Cost {
//                     cost: _,
//                     group: Some(g),
//                 } = cost
//                 {
//                     if *g == gr {
//                         extras.insert(*pos);
//                     }
//                 }
//             }
//         }
//         group
//     } else {
//         None
//     };
//     dbg!(&extras);
//     let path = astar(start, end, cost_map, 1, bounding_box, path_group)?;
//     let tiles = path_to_tile_chars(
//         &path,
//         &extras,
//         style,
//         line_style,
//         start_style,
//         start_direction,
//         end_style,
//         end_direction,
//         path_weight,
//     )?;
//     let mut result = tiles;
//     if let Some(arrow_style) = start_arrow_style {
//         if let Some(dir) = start_direction.or_else(|| delta_to_direction(path[1], path[0])) {
//             result.insert(
//                 path[0],
//                 Cell {
//                     character: arrow_style.arrow_char(dir),
//                     style: start_style,
//                     weight: path_weight,
//                 },
//             );
//         }
//     }
//     if let Some(arrow_style) = end_arrow_style {
//         if let Some(dir) =
//             end_direction.or_else(|| delta_to_direction(path[path.len() - 2], path[path.len() - 1]))
//         {
//             result.insert(
//                 path[path.len() - 1],
//                 Cell {
//                     character: arrow_style.arrow_char(dir),
//                     style: end_style,
//                     weight: path_weight,
//                 },
//             );
//         }
//     }
//     Ok(result.into_iter().collect())
// }
