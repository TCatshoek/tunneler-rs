use std::collections::HashMap;
use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_prototype_lyon::prelude::*;
use marching_squares::{Field, march};
use marching_squares::simplify::simplify;
use crate::mouse::MouseWorldPosition;

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
pub struct MapChunkData {
    data: Vec<MapDataValue>,
    width: usize,
    height: usize,
    chunk_pos: (i32, i32)
}

#[derive(Resource)]
pub struct MapChunkEntityGrid {
    data: HashMap<(i32, i32), Entity>,
}

impl MapChunkEntityGrid {
    fn new() -> Self {
        Self{
            data: HashMap::new()
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<Entity> {
        self.data.get(&(x, y)).and_then(|x| Some(x.clone()))
    }

    fn set(&mut self, x: i32, y: i32, val: Entity) {
        if self.data.contains_key(&(x, y)) {
            panic!("Chunk already exists at ({}, {})", x, y);
        }
        self.data.insert((x, y), val);
    }
}

pub fn setup_map(mut commands: Commands) {
    let mut chunk_grid = MapChunkEntityGrid::new();

    for x in 0..2 {
        for y in 0..2 {
            let mut map_data = MapChunkData::new(500, 500, x, y);

            map_data.fill(MapDataValue::Sand);

            let path = map_data.to_path();

            let id = commands.spawn((
                map_data,
                ShapeBundle {
                    path,
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz((x * 500) as f32, (y * 500) as f32, 0.0),
                        ..default()
                    },
                    ..default()
                },
                // Stroke::new(Color::BLACK, 1.0),
                Fill::color(Color::BISQUE),
                // NoFrustumCulling
            )).id();

            chunk_grid.set(x, y, id);
        }
    }

    commands.insert_resource(chunk_grid)
}

struct LineInterp {
    begin: (f32, f32),
    end: (f32, f32),
    steps: usize,
    step_size: f32,
    cur_pos: f32,
}

impl LineInterp {
    pub fn new(begin: (f32, f32), end: (f32, f32), steps: usize) -> Self {
        Self {
            begin,
            end,
            steps,
            step_size: 1.0 / steps as f32,
            cur_pos: 0.0,
        }
    }

    pub fn interp(&self, amount: f32) -> (f32, f32) {
        let diff_x = self.begin.0 - self.end.0;
        let diff_y = self.begin.1 - self.end.1;
        (self.begin.0 - (diff_x * amount), self.begin.1 - (diff_y * amount))
    }
}

impl Iterator for LineInterp {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        self.cur_pos += self.step_size;

        if self.cur_pos < 1.0 {
            Some(self.interp(self.cur_pos))
        } else {
            None
        }
    }
}

pub fn update_map(
    mut query: Query<(&mut MapChunkData, &mut Path, &Transform, Entity)>,
    mut events: EventReader<MouseWorldPosition>,
    buttons: Res<Input<MouseButton>>,
    chunk_grid: Res<MapChunkEntityGrid>,
    mut prev_mouse_pos: Local<(f32, f32)>,
) {
    let mut id = 0;
    if let Some(pos) = events.read().last() {
        for (mut mapdata, mut path, transform, entity) in query.iter_mut() {
            id += 1;

            let line = LineInterp::new(*prev_mouse_pos, (pos.x, pos.y), 100);

            for interp_pos in line {
                let map_x = (interp_pos.0.abs() - transform.translation.x) as usize;
                let map_y = (interp_pos.1.abs() - transform.translation.y) as usize;

                println!("[{}, {:?}] mouse map pos: {}, {}", id, entity, map_x, map_y);

                if mapdata.is_in_bounds(map_x, map_y) {
                    if buttons.pressed(MouseButton::Left) {
                        mapdata.set(map_x, map_y, MapDataValue::Sand);
                    }

                    if buttons.pressed(MouseButton::Right) {
                        mapdata.set(map_x, map_y, MapDataValue::Empty);
                    }
                }
            }
            // let cur = mapdata.chunk_pos;
            // let right = chunk_grid.get(cur.0 + 1, cur.1).unwrap();
            // let r = query.get(right);
            //
            // let top = chunk_grid.get(cur.0, cur.1 + 1);
            // let top_right = chunk_grid.get(cur.0 + 1, cur.1 + 1);

            *path = mapdata.to_path();
        }
        *prev_mouse_pos = (pos.x, pos.y);
    }
}


impl MapChunkData {
    pub fn new(width: usize, height: usize, pos_x: i32, pos_y: i32) -> MapChunkData {
        MapChunkData {
            data: vec![MapDataValue::Empty; width * height],
            width,
            height,
            chunk_pos: (pos_x, pos_y)
        }
    }

    pub fn fill(&mut self, val: MapDataValue) {
        self.data.fill(val);
    }

    #[inline]
    fn bounds_check(&self, x: usize, y: usize) {
        if !self.is_in_bounds(x, y) {
            panic!("Out of bounds map data access ({}, {})", x, y)
        }
    }

    #[inline]
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
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

    pub fn to_path(&self) -> Path {
        let start = Instant::now();

        let contours = march(self, 0.5);

        let mut pathbuilder = PathBuilder::new();

        let mut simplify_duration = Duration::default();
        for contour in contours {
            let start2 = Instant::now();
            let simplified_contour = simplify(&contour);
            simplify_duration += start2.elapsed();


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
        }
        let result = pathbuilder.build();

        let duration = start.elapsed();
        // println!("simplify took {:?}", simplify_duration);
        // println!("to_path took {:?}", duration);
        result
    }
}

impl Field for MapChunkData {
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

struct ConnectedMapChunkData<'a> {
    data: &'a MapChunkData,
    data_right: &'a MapChunkData,
    data_top: &'a MapChunkData,
    data_top_right: &'a MapChunkData
}
impl<'a> Field for ConnectedMapChunkData<'a> {
    fn dimensions(&self) -> (usize, usize) {
        (self.data.width, self.data.height)
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        let (w, h) = self.dimensions();

        if x == w.saturating_sub(1) && y == h.saturating_sub(1) {
            return self.data_top_right.z_at(0, 0);
        }

        if x == w.saturating_sub(1) {
            return self.data_right.z_at(0, y);
        }

        if y == h.saturating_sub(1) {
            return self.data_top.z_at(x, 0);
        }

        self.data.z_at(x, y)
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{MapChunkData, MapDataValue};

    #[test]
    fn access() {
        let mapdata = MapChunkData::new(5, 5);
        assert_eq!(mapdata.get(0, 0), MapDataValue::Empty);
        assert_eq!(mapdata.get(4, 4), MapDataValue::Empty);
    }

    #[test]
    fn set() {
        let mut mapdata = MapChunkData::new(5, 5);
        mapdata.set(0, 0, MapDataValue::Sand);
        mapdata.set(4, 4, MapDataValue::Sand);
        assert_eq!(mapdata.get(0, 0), MapDataValue::Sand);
        assert_eq!(mapdata.get(4, 4), MapDataValue::Sand);
        assert_eq!(mapdata.get(1, 1), MapDataValue::Empty);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_read_x() {
        let mapdata = MapChunkData::new(5, 5);
        mapdata.get(5, 0);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_read_y() {
        let mapdata = MapChunkData::new(5, 5);
        mapdata.get(0, 5);
    }
}