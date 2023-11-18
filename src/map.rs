use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use marching_squares::{Field, march};
use marching_squares::simplify::simplify;

#[repr(u8)]
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub enum MapDataValue {
    Empty = 0,
    Sand = 1,
    Rock = 2,
    Base = 3,
    Wall = 4,
}

#[derive(Component)]
pub struct MapData {
    data: Vec<MapDataValue>,
    width: usize,
    height: usize,
}

pub fn setup_map(mut commands: Commands) {
    let mut map_data = MapData::new(500, 500);

    for x in 200..300 {
        for y in 200..300 {
            map_data.set(x, y, MapDataValue::Sand);
        }
    }

    let paths = map_data.to_paths();

    commands.spawn((
        ShapeBundle {
            path: Path(paths[0].0.clone()),
            spatial: SpatialBundle {
                ..default()
            },
            ..default()
        },
        Stroke::new(Color::BLACK, 3.0),
        Fill::color(Color::RED)
    ));
}

impl MapData {
    pub fn new(width: usize, height: usize) -> MapData {
        MapData {
            data: vec![MapDataValue::Empty; width * height],
            width,
            height,
        }
    }

    #[inline]
    fn bounds_check(&self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds map data access ({}, {})", x, y)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> MapDataValue {
        self.bounds_check(x, y);
        unsafe {
            *self.data.get_unchecked(x + y * self.width)
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: MapDataValue) {
        self.bounds_check(x, y);
        self.data[x + y * self.width] = val
    }

    pub fn to_paths(&self) -> Vec<Path> {
        let contours = march(self, 0.5);

        let mut paths = Vec::new();

        for contour in contours {
            let mut pathbuilder = PathBuilder::new();

            let simplified_contour = simplify(&contour);

            let mut contour_iter = simplified_contour.iter();
            let first_point = contour_iter.next().unwrap();

            pathbuilder.move_to(Vec2::new(first_point.0 as f32,
                                          first_point.1 as f32));

            for point in contour_iter {
                let x = point.0 as f32;
                let y = point.1 as f32;

                pathbuilder.line_to(Vec2::new(x, y));
            }

            pathbuilder.close();

            let path = pathbuilder.build();
            paths.push(path);
        }

        paths
    }
}

impl Field for MapData {
    fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        match self.get(x, y) {
            MapDataValue::Empty => 0.0,
            _ => 1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{MapData, MapDataValue};

    #[test]
    fn access() {
        let mapdata = MapData::new(5, 5);
        assert_eq!(mapdata.get(0, 0), MapDataValue::Empty);
        assert_eq!(mapdata.get(4, 4), MapDataValue::Empty);
    }

    #[test]
    fn set() {
        let mut mapdata = MapData::new(5, 5);
        mapdata.set(0, 0, MapDataValue::Sand);
        mapdata.set(4, 4, MapDataValue::Sand);
        assert_eq!(mapdata.get(0, 0), MapDataValue::Sand);
        assert_eq!(mapdata.get(4, 4), MapDataValue::Sand);
        assert_eq!(mapdata.get(1, 1), MapDataValue::Empty);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_read_x() {
        let mapdata = MapData::new(5, 5);
        mapdata.get(5, 0);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_read_y() {
        let mapdata = MapData::new(5, 5);
        mapdata.get(0, 5);
    }
}