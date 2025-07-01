#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

use owo_colors::{Effect, OwoColorize, Style};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{PyList, PyTuple},
};
use regex::Regex;

#[pyclass]
#[derive(Default, Copy, Clone, Debug)]
struct BoundingBox {
    #[pyo3(get, set)]
    top: isize,
    #[pyo3(get, set)]
    right: isize,
    #[pyo3(get, set)]
    bottom: isize,
    #[pyo3(get, set)]
    left: isize,
}
impl BoundingBox {
    fn contains_point(&self, p: (isize, isize)) -> bool {
        p.0 >= self.left && p.0 <= self.right && p.1 >= self.bottom && p.1 <= self.top
    }
    fn contains_bounding_box(&self, bbox: BoundingBox) -> bool {
        bbox.left >= self.left
            && bbox.right <= self.right
            && bbox.bottom >= self.bottom
            && bbox.top <= self.top
    }
    fn as_map(
        &self,
        border_style: &TextStyle,
        fill_style: &TextStyle,
        line_style: Option<LineStyle>,
        weight: Option<usize>,
        transparent: bool,
    ) -> HashMap<(isize, isize), Pixel> {
        let mut pixels = HashMap::default();
        for i in self.left + 1..self.right {
            pixels.insert(
                (i, self.top),
                Pixel {
                    character: line_style.map_or(' ', |ls| ls.get_char((false, true, false, true))),
                    position: (i, self.top),
                    style: border_style.clone(),
                    weight,
                },
            );
            pixels.insert(
                (i, self.bottom),
                Pixel {
                    character: line_style.map_or(' ', |ls| ls.get_char((false, true, false, true))),
                    position: (i, self.bottom),
                    style: border_style.clone(),
                    weight,
                },
            );
        }
        for j in self.bottom + 1..self.top {
            pixels.insert(
                (self.left, j),
                Pixel {
                    character: line_style.map_or(' ', |ls| ls.get_char((true, false, true, false))),
                    position: (self.left, j),
                    style: border_style.clone(),
                    weight,
                },
            );
            pixels.insert(
                (self.right, j),
                Pixel {
                    character: line_style.map_or(' ', |ls| ls.get_char((true, false, true, false))),
                    position: (self.right, j),
                    style: border_style.clone(),
                    weight,
                },
            );
        }
        pixels.insert(
            (self.right, self.top),
            Pixel {
                character: line_style.map_or(' ', |ls| ls.get_char((false, false, true, true))),
                position: (self.right, self.top),
                style: border_style.clone(),
                weight,
            },
        );
        pixels.insert(
            (self.right, self.bottom),
            Pixel {
                character: line_style.map_or(' ', |ls| ls.get_char((true, false, false, true))),
                position: (self.right, self.bottom),
                style: border_style.clone(),
                weight,
            },
        );
        pixels.insert(
            (self.left, self.top),
            Pixel {
                character: line_style.map_or(' ', |ls| ls.get_char((false, true, true, false))),
                position: (self.left, self.top),
                style: border_style.clone(),
                weight,
            },
        );
        pixels.insert(
            (self.left, self.bottom),
            Pixel {
                character: line_style.map_or(' ', |ls| ls.get_char((true, true, false, false))),
                position: (self.left, self.bottom),
                style: border_style.clone(),
                weight,
            },
        );
        if !transparent {
            for i in self.left + 1..self.right {
                for j in self.bottom + 1..self.top {
                    pixels.insert(
                        (i, j),
                        Pixel {
                            character: ' ',
                            position: (i, j),
                            style: fill_style.clone(),
                            weight,
                        },
                    );
                }
            }
        }
        pixels
    }
}
#[pymethods]
impl BoundingBox {
    #[new]
    fn new(top: isize, right: isize, bottom: isize, left: isize) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
    fn __contains__(&self, other: Bound<PyAny>) -> PyResult<bool> {
        if let Ok(point) = other.extract::<(isize, isize)>() {
            Ok(self.contains_point(point))
        } else if let Ok(bbox) = other.extract::<BoundingBox>() {
            Ok(self.contains_bounding_box(bbox))
        } else {
            Err(PyTypeError::new_err(
                "Expected either a tuple[int, int] or a BoundingBox",
            ))
        }
    }
    fn __add__(&self, other: Bound<PyAny>) -> PyResult<BoundingBox> {
        if let Ok(point) = other.extract::<(isize, isize)>() {
            Ok(*self + point)
        } else if let Ok(bbox) = other.extract::<BoundingBox>() {
            Ok(*self + bbox)
        } else {
            Err(PyTypeError::new_err(
                "Expected either a tuple[int, int] or a BoundingBox",
            ))
        }
    }
    fn __str__(&self) -> String {
        format!(
            "BoundingBox(top={}, right={}, bottom={}, left={})",
            self.top, self.right, self.bottom, self.left
        )
    }
    #[getter]
    fn width(&self) -> usize {
        (self.right - self.left) as usize
    }
    #[getter]
    fn height(&self) -> usize {
        (self.top - self.bottom) as usize
    }
}
impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Self) -> Self::Output {
        BoundingBox {
            top: self.top.max(rhs.top),
            right: self.right.max(rhs.right),
            bottom: self.bottom.min(rhs.bottom),
            left: self.left.min(rhs.left),
        }
    }
}
impl AddAssign for BoundingBox {
    fn add_assign(&mut self, rhs: Self) {
        self.top = self.top.max(rhs.top);
        self.right = self.right.max(rhs.right);
        self.bottom = self.bottom.min(rhs.bottom);
        self.left = self.left.min(rhs.left);
    }
}
impl Add<(isize, isize)> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        BoundingBox {
            top: self.top.max(rhs.1),
            right: self.right.max(rhs.0),
            bottom: self.bottom.min(rhs.1),
            left: self.left.min(rhs.0),
        }
    }
}
impl AddAssign<(isize, isize)> for BoundingBox {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.top = self.top.max(rhs.1);
        self.right = self.right.max(rhs.0);
        self.bottom = self.bottom.min(rhs.1);
        self.left = self.left.min(rhs.0);
    }
}
impl From<(isize, isize, isize, isize)> for BoundingBox {
    fn from(value: (isize, isize, isize, isize)) -> Self {
        Self {
            top: value.0,
            right: value.1,
            bottom: value.2,
            left: value.3,
        }
    }
}
impl From<BoundingBox> for (isize, isize, isize, isize) {
    fn from(value: BoundingBox) -> Self {
        (value.top, value.right, value.bottom, value.left)
    }
}

#[pyclass(name = "Style")]
#[derive(Default, Clone, Debug)]
struct TextStyle {
    effects: HashSet<String>,
    fg: Option<color_art::Color>,
    bg: Option<color_art::Color>,
}
#[pymethods]
impl TextStyle {
    #[new]
    fn new(s: &str) -> PyResult<Self> {
        s.parse()
    }
    fn __add__(&self, obj: Bound<PyAny>) -> PyResult<Self> {
        Ok(self.clone() + obj.try_into()?)
    }
    fn __call__(&self, text: &str) -> PyResult<String> {
        self.render(text)
    }
    fn __str__(&self) -> String {
        format!(
            "Style(fg={}, bg={}, effects=[{}])",
            if let Some(fg) = self.fg {
                fg.hex()
            } else {
                "None".to_string()
            },
            if let Some(bg) = self.bg {
                bg.hex()
            } else {
                "None".to_string()
            },
            self.effects
                .clone()
                .into_iter()
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
impl TextStyle {
    fn render(&self, text: &str) -> PyResult<String> {
        let effects = self
            .effects
            .iter()
            .map(|style| match style.as_str() {
                "bold" => Ok(Effect::Bold),
                "dimmed" => Ok(Effect::Dimmed),
                "italic" => Ok(Effect::Italic),
                "underline" => Ok(Effect::Underline),
                "blink" => Ok(Effect::Blink),
                "blinkfast" => Ok(Effect::BlinkFast),
                "reversed" => Ok(Effect::Reversed),
                "hidden" => Ok(Effect::Hidden),
                "strikethrough" => Ok(Effect::Strikethrough),
                _ => unreachable!(),
            })
            .collect::<PyResult<Vec<_>>>()?;
        let mut style = Style::new().effects(&effects);
        if let Some(fg_col) = self.fg {
            style = style.truecolor(fg_col.red(), fg_col.green(), fg_col.blue());
        }
        if let Some(bg_col) = self.bg {
            style = style.on_truecolor(bg_col.red(), bg_col.green(), bg_col.blue());
        }
        Ok(text.style(style).to_string())
    }
}
impl<'py> TryFrom<Bound<'py, PyAny>> for TextStyle {
    type Error = PyErr;

    fn try_from(value: Bound<PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = value.extract::<String>() {
            s.parse()
        } else if let Ok(ts) = value.extract::<TextStyle>() {
            Ok(ts)
        } else {
            Err(PyTypeError::new_err("Expected either a str or a Style"))
        }
    }
}
impl Add for TextStyle {
    type Output = TextStyle;

    fn add(self, rhs: Self) -> Self::Output {
        let mut effects = self.effects;
        effects.extend(rhs.effects);
        let fg = rhs.fg.or(self.fg);
        let bg = rhs.bg.or(self.bg);
        Self { effects, fg, bg }
    }
}
impl AddAssign for TextStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.effects.extend(rhs.effects);
        self.fg = rhs.fg.or(self.fg);
        self.bg = rhs.bg.or(self.bg);
    }
}
impl FromStr for TextStyle {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let all_effects = [
            "bold",
            "dimmed",
            "italic",
            "underline",
            "blinkfast",
            "blink",
            "reversed",
            "hidden",
            "strikethrough",
        ];
        let effects_re = all_effects.join("|");
        let re = Regex::new(&format!(
            r"^(?P<styles>(?:({effects_re})\s*)*)?(?P<fg>#[\da-f]{{6}}|\w+)?(?:\s*on\s+(?P<bg>#[\da-f]{{6}}|\w+))?$"
)).unwrap();
        let mut effects = HashSet::new();
        if let Some(captures) = re.captures(s.to_lowercase().trim()) {
            if let Some(matched_effects) = captures.name("styles") {
                effects = matched_effects
                    .as_str()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>();
            }
            let mut fg = None;
            if let Some(fg_str) = captures.name("fg").map(|m| m.as_str()) {
                let c: color_art::Color = fg_str
                    .parse()
                    .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
                fg = Some(c);
            }
            let mut bg = None;
            if let Some(bg_str) = captures.name("bg").map(|m| m.as_str()) {
                let c: color_art::Color = bg_str
                    .parse()
                    .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
                bg = Some(c);
            }
            return Ok(TextStyle { effects, fg, bg });
        }
        Err(PyValueError::new_err("Failed to parse style string"))
    }
}

#[pyclass]
#[derive(Clone, Debug)]
struct Pixel {
    #[pyo3(get, set)]
    character: char,
    #[pyo3(get, set)]
    position: (isize, isize),
    #[pyo3(get, set)]
    style: TextStyle,
    #[pyo3(get, set)]
    weight: Option<usize>,
}
#[pymethods]
impl Pixel {
    #[new]
    #[pyo3(signature = (character, position = None, style = None, *, weight = None))]
    fn new(
        character: char,
        position: Option<(isize, isize)>,
        style: Option<String>,
        weight: Option<usize>,
    ) -> PyResult<Self> {
        Ok(Self {
            character,
            position: position.unwrap_or_default(),
            style: style.unwrap_or_default().parse()?,
            weight,
        })
    }
    fn at(&self, position: (isize, isize)) -> Self {
        Self {
            character: self.character,
            position,
            style: self.style.clone(),
            weight: self.weight,
        }
    }
    fn __str__(&self) -> PyResult<String> {
        self.render()
    }
    fn render(&self) -> PyResult<String> {
        self.style.render(&self.character.to_string())
    }
}
impl Pixel {
    fn with_weight(&self, weight: Option<usize>) -> Self {
        let mut new_pixel = self.clone();
        new_pixel.weight = weight;
        new_pixel
    }
}

#[pyclass(sequence)]
#[derive(Clone)]
struct Group {
    #[pyo3(get, set)]
    pixels: Vec<Pixel>,
    position: (isize, isize),
    style: TextStyle,
    weight: Option<usize>,
}
#[pymethods]
impl Group {
    #[new]
    #[pyo3(signature = (pixels, position = None, style = None, *, weight = 0))]
    fn new(
        pixels: Vec<Pixel>,
        position: Option<(isize, isize)>,
        style: Option<String>,
        weight: Option<usize>,
    ) -> PyResult<Self> {
        Ok(Self {
            pixels,
            position: position.unwrap_or_default(),
            style: style.unwrap_or_default().parse()?,
            weight,
        })
    }
    fn __len__(&self) -> usize {
        self.pixels.len()
    }
    fn __getitem__(&self, index: usize) -> Pixel {
        self.pixels[index].clone()
    }
    fn __setitem__(&mut self, index: usize, value: Pixel) {
        self.pixels[index] = value;
    }
    fn at(&self, position: (isize, isize)) -> Self {
        Self {
            pixels: self.pixels.clone(),
            position,
            style: self.style.clone(),
            weight: self.weight,
        }
    }
}

fn objs_to_map(args: &Bound<'_, PyAny>) -> PyResult<HashMap<(isize, isize), Pixel>> {
    let mut map: HashMap<(isize, isize), Pixel> = HashMap::new();
    let objs: Vec<Bound<PyAny>> = if let Ok(it) = args.downcast::<PyTuple>() {
        it.iter().collect()
    } else if let Ok(it) = args.downcast::<PyList>() {
        it.iter().collect()
    } else {
        return Err(PyTypeError::new_err("Expected either a list or a tuple"));
    };
    for obj in objs {
        if let Ok(pixel) = obj.extract::<Pixel>() {
            map.insert(pixel.position, pixel);
        } else if let Ok(group) = obj.extract::<Group>() {
            for p in &group.pixels {
                let mut new_pixel = p.clone();
                let x = new_pixel.position.0 + group.position.0;
                let y = new_pixel.position.1 + group.position.1;
                new_pixel.position = (x, y);
                new_pixel.style += group.style.clone();
                new_pixel.weight = match (new_pixel.weight, group.weight) {
                    (None, _) | (_, None) => None,
                    (Some(w1), Some(w2)) => Some(w1 + w2),
                };
                map.insert(new_pixel.position, new_pixel);
            }
        } else if let Ok(textpath) = obj.extract::<TextPath>() {
            let group = textpath.as_group()?;
            for p in &group.pixels {
                let mut new_pixel = p.clone();
                let x = new_pixel.position.0 + group.position.0;
                let y = new_pixel.position.1 + group.position.1;
                new_pixel.position = (x, y);
                new_pixel.style += group.style.clone();
                new_pixel.weight = match (new_pixel.weight, group.weight) {
                    (None, _) | (_, None) => None,
                    (Some(w1), Some(w2)) => Some(w1 + w2),
                };
                map.insert(new_pixel.position, new_pixel);
            }
        } else if let Ok(textbox) = obj.extract::<Box>() {
            let group = textbox.as_group();
            for p in &group.pixels {
                let mut new_pixel = p.clone();
                let x = new_pixel.position.0 + group.position.0;
                let y = new_pixel.position.1 + group.position.1;
                new_pixel.position = (x, y);
                new_pixel.style += group.style.clone();
                new_pixel.weight = match (new_pixel.weight, group.weight) {
                    (None, _) | (_, None) => None,
                    (Some(w1), Some(w2)) => Some(w1 + w2),
                };
                map.insert(new_pixel.position, new_pixel);
            }
        } else {
            return Err(PyTypeError::new_err(
                "Expected either Pixels, Groups, TextPaths, or Boxes as arguments",
            ));
        }
    }
    Ok(map)
}

fn map_to_bounding_box(map: &HashMap<(isize, isize), Pixel>) -> BoundingBox {
    let min_x = map.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let min_y = map.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let max_x = map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let max_y = map.keys().map(|(_, y)| *y).max().unwrap_or_default();
    BoundingBox {
        top: max_y,
        right: max_x,
        bottom: min_y,
        left: min_x,
    }
}
fn pixels_to_bounding_box(pixels: &[Pixel]) -> BoundingBox {
    BoundingBox {
        top: pixels
            .iter()
            .map(|p| p.position.1)
            .max()
            .unwrap_or_default(),
        right: pixels
            .iter()
            .map(|p| p.position.0)
            .max()
            .unwrap_or_default(),
        bottom: pixels
            .iter()
            .map(|p| p.position.1)
            .min()
            .unwrap_or_default(),
        left: pixels
            .iter()
            .map(|p| p.position.0)
            .min()
            .unwrap_or_default(),
    }
}

#[pyfunction(signature = (*args))]
fn render(args: &Bound<'_, PyTuple>) -> PyResult<String> {
    let map = objs_to_map(args)?;
    let bb = map_to_bounding_box(&map);
    let mut output = String::new();
    for y in (bb.bottom..=bb.top).rev() {
        for x in bb.left..=bb.right {
            if let Some(p) = map.get(&(x, y)) {
                output.push_str(&p.render()?);
            } else {
                output.push(' ');
            }
        }
        output.push('\n')
    }
    Ok(output)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "up",
                Direction::Right => "right",
                Direction::Down => "down",
                Direction::Left => "left",
            }
        )
    }
}
impl FromStr for Direction {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" => Ok(Direction::Up),
            "right" => Ok(Direction::Right),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            _ => Err(PyValueError::new_err("Invalid direction")),
        }
    }
}

#[derive(Clone, Copy, Default)]
enum LineStyle {
    #[default]
    Regular,
    Thick,
    Double,
}
impl LineStyle {
    fn get_char(&self, neighbors: (bool, bool, bool, bool)) -> char {
        let chars: Vec<char> = match self {
            LineStyle::Regular => " ╴╷┐╶─┌┬╵┘│┤└┴├┼",
            LineStyle::Thick => " ━┃┓━━┏┳┃┛┃┫┗┻┣╋",
            LineStyle::Double => " ═║╗══╔╦║╝║╣╚╩╠╬",
        }
        .to_string()
        .chars()
        .collect();
        let index = (neighbors.0 as usize) << 3
            | (neighbors.1 as usize) << 2
            | (neighbors.2 as usize) << 1
            | (neighbors.3 as usize);
        chars[index]
    }
}
impl Display for LineStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LineStyle::Regular => "regular",
                LineStyle::Thick => "thick",
                LineStyle::Double => "double",
            }
        )
    }
}
impl FromStr for LineStyle {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "regular" => Ok(LineStyle::Regular),
            "thick" => Ok(LineStyle::Thick),
            "double" => Ok(LineStyle::Double),
            other => Err(PyValueError::new_err(format!(
                "Invalid line style [{}]",
                other
            ))),
        }
    }
}

#[derive(Clone, Copy, Default)]
enum ArrowType {
    #[default]
    Arrow,
    OpenArrow,
    Custom {
        up: char,
        right: char,
        down: char,
        left: char,
    },
}
impl ArrowType {
    fn render(&self, direction: &Direction) -> String {
        match self {
            ArrowType::Arrow => match direction {
                Direction::Up => "▲",
                Direction::Right => "▶",
                Direction::Down => "▼",
                Direction::Left => "◀",
            }
            .to_string(),
            ArrowType::OpenArrow => match direction {
                Direction::Up => "△",
                Direction::Right => "▷",
                Direction::Down => "▽",
                Direction::Left => "◁",
            }
            .to_string(),
            ArrowType::Custom {
                up,
                right,
                down,
                left,
            } => match direction {
                Direction::Up => up,
                Direction::Right => right,
                Direction::Down => down,
                Direction::Left => left,
            }
            .to_string(),
        }
    }
}

#[pyfunction]
fn arrow(s: &str) -> PyResult<String> {
    let all_arrow_types = ["openarrow", "arrow", "custom"];
    let arrow_types_re = all_arrow_types.join("|");
    let re = Regex::new(&format!(
            r"^(?P<direction>(?:(up|right|down|left))\s*)?(?P<arrows>(?:({arrow_types_re}))|(custom\[(?P<customs>.{{4}})\])|(custom\[(?P<custom>.)\])\s*)?$"
)).unwrap();
    let mut direction = Direction::default();
    let mut arrow = ArrowType::default();
    if let Some(captures) = re.captures(s.to_lowercase().trim()) {
        if let Some(matched_direction) = captures.name("direction") {
            match matched_direction.as_str() {
                "up" => direction = Direction::Up,
                "right" => direction = Direction::Right,
                "down" => direction = Direction::Down,
                "left" => direction = Direction::Left,
                _ => unreachable!(),
            }
        }
        if let Some(matched_arrow) = captures.name("arrows") {
            match matched_arrow.as_str() {
                "arrow" => arrow = ArrowType::Arrow,
                "openarrow" => arrow = ArrowType::OpenArrow,
                _ => {
                    if let Some(matched_custom) = captures.name("custom") {
                        let custom_char = matched_custom.as_str().chars().next().unwrap();
                        arrow = ArrowType::Custom {
                            up: custom_char,
                            right: custom_char,
                            down: custom_char,
                            left: custom_char,
                        }
                    } else if let Some(matched_customs) = captures.name("customs") {
                        let custom_chars = matched_customs.as_str().chars().collect::<Vec<char>>();
                        arrow = ArrowType::Custom {
                            up: custom_chars[0],
                            right: custom_chars[1],
                            down: custom_chars[2],
                            left: custom_chars[3],
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        return Ok(arrow.render(&direction));
    }
    Err(PyValueError::new_err("Failed to parse style string"))
}

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (isize, isize),
    dir: Option<Direction>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[pyclass]
#[derive(Clone)]
struct TextPath {
    path: Vec<(isize, isize)>,
    #[pyo3(get, set)]
    style: TextStyle,
    line_style: LineStyle,
    #[pyo3(get, set)]
    weight: Option<usize>,
    start_direction: Option<Direction>,
    end_direction: Option<Direction>,
    start: (isize, isize),
    end: (isize, isize),
    paths: HashMap<(isize, isize), Pixel>,
}

#[pymethods]
impl TextPath {
    #[new]
    #[pyo3(signature = (start, end, style = None, *, line_style = "regular".to_string(), weight = None, start_direction = None, end_direction = None, bend_penalty = 1, environment = None, barriers = None, paths = None))]
    fn new(
        py: Python,
        start: (isize, isize),
        end: (isize, isize),
        style: Option<String>,
        line_style: String,
        weight: Option<usize>,
        start_direction: Option<String>,
        end_direction: Option<String>,
        bend_penalty: usize,
        environment: Option<Bound<'_, PyAny>>,
        barriers: Option<Bound<'_, PyAny>>,
        paths: Option<Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let mut environment =
            objs_to_map(&environment.unwrap_or(PyTuple::empty(py).as_any().clone()))?;
        for (position, pixel) in
            objs_to_map(&barriers.unwrap_or(PyTuple::empty(py).as_any().clone()))?
        {
            environment.insert(position, pixel.with_weight(None));
        }
        let paths = objs_to_map(&paths.unwrap_or(PyTuple::empty(py).as_any().clone()))?;
        for (position, pixel) in &paths {
            environment.insert(*position, pixel.with_weight(Some(0)));
        }
        let mut bb = map_to_bounding_box(&environment);
        bb += start;
        bb += end;
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(State {
            cost: 0,
            pos: start,
            dir: None,
        });
        cost_so_far.insert(start, 0);
        while let Some(State { cost, pos, dir }) = heap.pop() {
            if pos == end {
                let mut path = vec![pos];
                let mut current = (pos, dir);
                while let Some(&(previous_pos, previous_dir)) = came_from.get(&current) {
                    path.push(previous_pos);
                    current = (previous_pos, previous_dir);
                }
                path.reverse();
                return Ok(Self {
                    path,
                    style: style.unwrap_or_default().parse()?,
                    line_style: line_style.parse()?,
                    weight,
                    start_direction: start_direction.map(|s| s.parse().unwrap()),
                    end_direction: end_direction.map(|s| s.parse().unwrap()),
                    start,
                    end,
                    paths,
                });
            }

            for new_dir in Direction::all() {
                let (dx, dy) = new_dir.delta();
                let next = (pos.0 + dx, pos.1 + dy);
                if !bb.contains_point(next) {
                    continue;
                }
                let weight = match environment.get(&next) {
                    Some(Pixel { weight: None, .. }) => continue,
                    Some(Pixel {
                        weight: Some(w), ..
                    }) => *w,
                    None => 1,
                };
                let bend_cost = if Some(new_dir) != dir && dir.is_some() {
                    bend_penalty
                } else {
                    0
                };
                let new_cost = cost + weight + bend_cost;
                let entry = cost_so_far.entry(next).or_insert(usize::MAX);
                let heuristic = |pos: (isize, isize)| -> usize {
                    ((end.0 - pos.0).abs() + (end.1 - pos.1).abs()) as usize
                };
                if new_cost < *entry {
                    *entry = new_cost;
                    heap.push(State {
                        cost: new_cost + heuristic(next),
                        pos: next,
                        dir: Some(new_dir),
                    });
                    came_from.insert((next, Some(new_dir)), (pos, dir));
                }
            }
        }
        Err(PyValueError::new_err("No path found"))
    }
    #[getter]
    fn get_start_direction(&self) -> String {
        self.start_direction
            .map_or("None".to_string(), |d| d.to_string())
    }
    #[setter]
    fn set_start_direction(&mut self, start_direction: Option<String>) -> PyResult<()> {
        self.start_direction = start_direction.map(|s| s.parse()).transpose()?;
        Ok(())
    }
    #[getter]
    fn get_end_direction(&self) -> String {
        self.end_direction
            .map_or("None".to_string(), |d| d.to_string())
    }
    #[setter]
    fn set_end_direction(&mut self, end_direction: Option<String>) -> PyResult<()> {
        self.end_direction = end_direction.map(|s| s.parse()).transpose()?;
        Ok(())
    }
    #[getter]
    fn get_line_style(&self) -> String {
        self.line_style.to_string()
    }
    #[setter]
    fn set_line_style(&mut self, line_style: String) -> PyResult<()> {
        self.line_style = line_style.parse()?;
        Ok(())
    }
}
impl TextPath {
    fn as_group(&self) -> PyResult<Group> {
        let mut path_map: HashSet<(isize, isize)> = self.path.clone().into_iter().collect();
        for (pos, _) in self.paths.iter() {
            path_map.insert(*pos);
        }
        if let Some(start_dir) = self.start_direction {
            path_map.insert((
                self.start.0 + start_dir.delta().0,
                self.start.1 + start_dir.delta().1,
            ));
        }
        if let Some(end_dir) = self.end_direction {
            path_map.insert((
                self.end.0 + end_dir.delta().0,
                self.end.1 + end_dir.delta().1,
            ));
        }
        let path_neighbors: Vec<(bool, bool, bool, bool)> = self
            .path
            .iter()
            .map(|pos| {
                (
                    path_map.contains(&(pos.0, pos.1 + 1)),
                    path_map.contains(&(pos.0 + 1, pos.1)),
                    path_map.contains(&(pos.0, pos.1 - 1)),
                    path_map.contains(&(pos.0 - 1, pos.1)),
                )
            })
            .collect();
        let pixels = self
            .path
            .iter()
            .zip(path_neighbors)
            .map(|(pos, n)| Pixel {
                character: self.line_style.get_char(n),
                position: *pos,
                weight: self.weight,
                style: self.style.clone(),
            })
            .collect();
        Ok(Group {
            pixels,
            position: (0, 0),
            style: TextStyle::default(),
            weight: Some(0),
        })
    }
}

#[derive(Clone, Copy, Default)]
enum Alignment {
    #[default]
    Top,
    Center,
    Bottom,
}
impl FromStr for Alignment {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(Alignment::Top),
            "center" => Ok(Alignment::Center),
            "bottom" => Ok(Alignment::Bottom),
            _ => Err(PyValueError::new_err("Invalid alignment")),
        }
    }
}
impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Alignment::Top => "top",
                Alignment::Center => "center",
                Alignment::Bottom => "bottom",
            }
        )
    }
}
#[derive(Clone, Copy, Default)]
enum Justification {
    #[default]
    Right,
    Center,
    Left,
}
impl FromStr for Justification {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "right" => Ok(Justification::Right),
            "center" => Ok(Justification::Center),
            "left" => Ok(Justification::Left),
            _ => Err(PyValueError::new_err("Invalid alignment")),
        }
    }
}
impl Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Justification::Right => "right",
                Justification::Center => "center",
                Justification::Left => "left",
            }
        )
    }
}

#[pyclass]
#[derive(Clone)]
struct Box {
    #[pyo3(get, set)]
    text: String,
    #[pyo3(get, set)]
    position: (isize, isize),
    #[pyo3(get, set)]
    width: Option<usize>,
    #[pyo3(get, set)]
    height: Option<usize>,
    #[pyo3(get, set)]
    style: TextStyle,
    #[pyo3(get, set)]
    border_style: TextStyle,
    line_style: Option<LineStyle>,
    #[pyo3(get, set)]
    weight: Option<usize>,
    #[pyo3(get, set)]
    padding: Option<(usize, usize, usize, usize)>,
    #[pyo3(get, set)]
    padding_style: TextStyle,
    align: Alignment,
    justify: Justification,
    #[pyo3(get, set)]
    truncate_string: Option<String>,
    #[pyo3(get, set)]
    transparent: bool,
    #[pyo3(get, set)]
    transparent_padding: bool,
    bbox: BoundingBox,
}
#[pymethods]
impl Box {
    #[new]
    #[pyo3(signature = (text = "", position = (0, 0), width = None, height = None, style = None, border_style = None, line_style = Some("regular".to_string()), weight = 1, padding = (0, 1, 0, 1), padding_style = None, align = "top", justify= "left", truncate_string = None, transparent = false, transparent_padding = false))]
    fn new(
        text: &str,
        position: (isize, isize),
        width: Option<usize>,
        height: Option<usize>,
        style: Option<String>,
        border_style: Option<String>,
        line_style: Option<String>,
        weight: Option<usize>,
        padding: Option<(usize, usize, usize, usize)>,
        padding_style: Option<String>,
        align: &str,
        justify: &str,
        truncate_string: Option<String>,
        transparent: bool,
        transparent_padding: bool,
    ) -> PyResult<Self> {
        Ok(Self {
            text: text.to_string(),
            position,
            width,
            height,
            style: style.unwrap_or_default().parse()?,
            border_style: border_style.unwrap_or_default().parse()?,
            line_style: line_style.map(|s| s.parse()).transpose()?,
            weight,
            padding,
            padding_style: padding_style.unwrap_or_default().parse()?,
            align: align.parse()?,
            justify: justify.parse()?,
            truncate_string,
            transparent,
            transparent_padding,
            bbox: BoundingBox::default(),
        })
    }
    #[getter]
    fn get_line_style(&self) -> Option<String> {
        self.line_style.map(|s| s.to_string())
    }
    #[setter]
    fn set_line_style(&mut self, line_style: Option<String>) -> PyResult<()> {
        self.line_style = line_style.map(|s| s.parse()).transpose()?;
        Ok(())
    }
    #[getter]
    fn get_align(&self) -> String {
        self.align.to_string()
    }
    #[setter]
    fn set_align(&mut self, align: String) -> PyResult<()> {
        self.align = align.parse()?;
        Ok(())
    }
    #[getter]
    fn get_justify(&self) -> String {
        self.justify.to_string()
    }
    #[setter]
    fn set_justify(&mut self, justify: String) -> PyResult<()> {
        self.justify = justify.parse()?;
        Ok(())
    }
    #[getter]
    fn get_bbox(&self) -> BoundingBox {
        let (_, bb_text) = self.format_text();
        let padding = self.padding.unwrap_or_default();
        BoundingBox::new(
            bb_text.top + padding.0 as isize + 1,
            bb_text.right + padding.1 as isize + 1,
            bb_text.bottom - padding.2 as isize - 1,
            bb_text.left - padding.3 as isize - 1,
        )
    }
}
impl Box {
    fn as_group(&self) -> Group {
        let (text_pixels, bb_text) = self.format_text();
        let padding = self.padding.unwrap_or_default();
        let bb_border = BoundingBox::new(
            bb_text.top + padding.0 as isize + 1,
            bb_text.right + padding.1 as isize + 1,
            bb_text.bottom - padding.2 as isize - 1,
            bb_text.left - padding.3 as isize - 1,
        );
        let mut pixels: HashMap<(isize, isize), Pixel> = bb_border.as_map(
            &self.border_style,
            &self.padding_style,
            self.line_style,
            self.weight,
            self.transparent_padding,
        );
        pixels.extend(text_pixels);
        Group {
            pixels: pixels.values().cloned().collect(),
            position: (0, 0),
            style: TextStyle::default(),
            weight: self.weight,
        }
    }
    fn format_text(&self) -> (HashMap<(isize, isize), Pixel>, BoundingBox) {
        let trunc = self.truncate_string.clone().unwrap_or("".to_string());

        // Step 1: Break input into lines and wrap each line individually
        let mut raw_lines = Vec::new(); // MODIFIED: single flat vector

        for line in self.text.lines() {
            if let Some(w) = self.width {
                let mut current = String::new();
                for word in line.split_whitespace() {
                    if current.len() + word.len() + if current.is_empty() { 0 } else { 1 } > w {
                        if !current.is_empty() {
                            raw_lines.push(current.clone()); // MODIFIED: avoid blank segments
                            current.clear();
                        }
                        current.push_str(word);
                    } else {
                        if !current.is_empty() {
                            current.push(' ');
                        }
                        current.push_str(word);
                    }
                }
                if !current.is_empty() || line.trim().is_empty() {
                    raw_lines.push(current); // MODIFIED: push empty lines too if explicitly in input
                }
            } else {
                raw_lines.push(line.to_string());
            }
        }
        // Step 2: Determine effective width and apply horizontal truncation
        let effective_width = self
            .width
            .unwrap_or_else(|| raw_lines.iter().map(|l| l.len()).max().unwrap_or(0));
        for line in raw_lines.iter_mut() {
            if line.len() > effective_width {
                if !trunc.is_empty() && trunc.len() <= effective_width {
                    line.truncate(effective_width - trunc.len());
                    line.push_str(&trunc);
                } else {
                    line.truncate(effective_width);
                }
            }
        }

        // Step 3: Determine height and apply vertical truncation
        let default_height = raw_lines.len().max(1);
        let effective_height = self.height.unwrap_or(default_height);
        if raw_lines.len() > effective_height {
            raw_lines.truncate(effective_height);
        }

        // Step 4: Pad each line horizontally based on justification
        let pad_line = |line: &str| -> Vec<Option<String>> {
            let padding = effective_width.saturating_sub(line.len());
            let (left_pad, right_pad) = match self.justify {
                Justification::Left => (0, padding),
                Justification::Right => (padding, 0),
                Justification::Center => (padding / 2, padding - padding / 2),
            };
            let mut row = vec![None; left_pad];
            row.extend(line.chars().map(|c| Some(c.to_string())));
            row.extend(std::iter::repeat(None).take(right_pad));
            row
        };

        let padded_lines: Vec<Vec<Option<String>>> =
            raw_lines.iter().map(|l| pad_line(l)).collect();

        // Step 5: Add vertical alignment (correct top-to-bottom ordering)
        let blank_row: Vec<Option<String>> = vec![None; effective_width];
        let vertical_padding = effective_height.saturating_sub(padded_lines.len());
        let (top_pad, bottom_pad) = match self.align {
            Alignment::Top => (0, vertical_padding),
            Alignment::Bottom => (vertical_padding, 0),
            Alignment::Center => (
                vertical_padding / 2,
                vertical_padding - vertical_padding / 2,
            ),
        };

        let mut result = Vec::new();
        result.extend(std::iter::repeat(blank_row.clone()).take(bottom_pad));
        result.extend(padded_lines.into_iter().rev().collect::<Vec<_>>());
        result.extend(std::iter::repeat(blank_row).take(top_pad));

        (
            result
                .iter()
                .enumerate()
                .map(|(j, chars)| {
                    chars
                        .iter()
                        .enumerate()
                        .filter_map(|(i, c)| match c {
                            Some(chr) => Some(Pixel {
                                character: chr.chars().collect::<Vec<char>>()[0],
                                position: (
                                    self.position.0 + i as isize,
                                    self.position.1 + j as isize,
                                ),
                                style: self.style.clone(),
                                weight: self.weight,
                            }),
                            None => {
                                if self.transparent {
                                    None
                                } else {
                                    Some(Pixel {
                                        character: ' ',
                                        position: (
                                            self.position.0 + i as isize,
                                            self.position.1 + j as isize,
                                        ),
                                        style: self.style.clone(),
                                        weight: self.weight,
                                    })
                                }
                            }
                        })
                        .collect::<Vec<Pixel>>()
                })
                .flatten()
                .map(|p| ((p.position, p)))
                .collect(),
            BoundingBox {
                top: self.position.1 + effective_height as isize - 1,
                right: self.position.0 + effective_width as isize - 1,
                bottom: self.position.1,
                left: self.position.0,
            },
        )
    }
}

#[pymodule]
fn textdraw(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<BoundingBox>()?;
    m.add_class::<Group>()?;
    m.add_class::<TextStyle>()?;
    m.add_class::<Pixel>()?;
    m.add_function(wrap_pyfunction!(render, m)?)?;
    m.add_function(wrap_pyfunction!(arrow, m)?)?;
    m.add_class::<TextPath>()?;
    m.add_class::<Box>()?;
    Ok(())
}
