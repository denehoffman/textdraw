#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

use owo_colors::{Effect, OwoColorize, Style};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::PyTuple,
};
use regex::Regex;

mod astar;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub(crate) struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<Point> for (isize, isize) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[pyclass]
#[derive(Default, Copy, Clone)]
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
    fn contains_point(&self, p: Point) -> bool {
        p.x >= self.left && p.x <= self.right && p.y >= self.bottom && p.y <= self.top
    }
    fn contains_bounding_box(&self, bbox: BoundingBox) -> bool {
        bbox.left >= self.left
            && bbox.right <= self.right
            && bbox.bottom >= self.bottom
            && bbox.top <= self.top
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
            Ok(self.contains_point(point.into()))
        } else if let Ok(bbox) = other.extract::<BoundingBox>() {
            Ok(self.contains_bounding_box(bbox))
        } else {
            Err(PyTypeError::new_err(
                "Expected either a Point or a BoundingBox",
            ))
        }
    }
    fn __add__(&self, other: Bound<PyAny>) -> PyResult<BoundingBox> {
        if let Ok(point) = other.extract::<(isize, isize)>() {
            Ok(*self + Point::from(point))
        } else if let Ok(bbox) = other.extract::<BoundingBox>() {
            Ok(*self + bbox)
        } else {
            Err(PyTypeError::new_err(
                "Expected either a Point or a BoundingBox",
            ))
        }
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
impl Add<Point> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Point) -> Self::Output {
        BoundingBox {
            top: self.top.max(rhs.y),
            right: self.right.max(rhs.x),
            bottom: self.bottom.min(rhs.y),
            left: self.left.min(rhs.x),
        }
    }
}
impl AddAssign<Point> for BoundingBox {
    fn add_assign(&mut self, rhs: Point) {
        self.top = self.top.max(rhs.y);
        self.right = self.right.max(rhs.x);
        self.bottom = self.bottom.min(rhs.y);
        self.left = self.left.min(rhs.x);
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
                other => Err(PyValueError::new_err(format!("Unknown effect {}", other,))),
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
            Err(PyTypeError::new_err("Expected either a str or a TextStyle"))
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
                    .collect::<HashSet<String>>()
                    .iter()
                    .map(|effect| {
                        if all_effects.contains(&effect.as_str()) {
                            Ok(effect.to_owned())
                        } else {
                            Err(PyValueError::new_err(format!(
                                "Unknown effect {}, (valid options are [{}]",
                                effect,
                                all_effects.join(", ")
                            )))
                        }
                    })
                    .collect::<Result<HashSet<String>, Self::Err>>()?;
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
    #[pyo3(signature = (pixels, position = None, style = None, *, weight = None))]
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

#[pyfunction(signature = (*args))]
fn render(args: &Bound<'_, PyTuple>) -> PyResult<String> {
    let mut map: HashMap<(isize, isize), Pixel> = HashMap::new();
    for obj in args.iter() {
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
        } else {
            return Err(PyTypeError::new_err(
                "Expected either Pixels or Groups as arguments",
            ));
        }
    }
    let min_x = map.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let min_y = map.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let max_x = map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let max_y = map.keys().map(|(_, y)| *y).max().unwrap_or_default();
    let mut output = String::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
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

#[pymodule]
fn textdraw(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<BoundingBox>()?;
    m.add_class::<Group>()?;
    m.add_class::<TextStyle>()?;
    m.add_class::<Pixel>()?;
    m.add_function(wrap_pyfunction!(render, m)?)?;
    Ok(())
}
