use std::collections::{BTreeMap, HashMap};
use std::cmp::Reverse;

use crate::game_data::screen::widget::world_rendering::{area_rendering_manager::ray_caster::casted_triangle::CastedTriangle};



pub type TileKey = u64;
pub type DepthKey = i32; // swap for your actual depth type

#[inline]
pub fn pack_tile_key(coord: [i32; 2]) -> TileKey {
    ((coord[0] as u32 as u64) << 32) | (coord[1] as u32 as u64)
}

#[inline]
pub fn unpack_tile_key(key: TileKey) -> [i32; 2] {
    [(key >> 32) as u32 as i32, key as u32 as i32]
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriangleSide {
    Left,
    Right,
}

pub struct FlattenedTileLairManager {
    // Reverse<DepthKey> so iteration order is highest depth first
    left_triangle: HashMap<TileKey, BTreeMap<Reverse<DepthKey>, FlattenedTriangleLair>>,
    right_triangle: HashMap<TileKey, BTreeMap<Reverse<DepthKey>, FlattenedTriangleLair>>,
}

impl FlattenedTileLairManager {
    pub fn new() -> Self {
        Self {
            left_triangle: HashMap::new(),
            right_triangle: HashMap::new(),
        }
    }

    // --- Insertion ---

    pub fn insert_triangle(
        &mut self,
        tile_key: TileKey,
        side: TriangleSide,
        depth: DepthKey,
        triangle: FlattenedTriangleLair,
    ) {
        let map = match side {
            TriangleSide::Left => &mut self.left_triangle,
            TriangleSide::Right => &mut self.right_triangle,
        };
        map.entry(tile_key)
            .or_insert_with(BTreeMap::new)
            .insert(Reverse(depth), triangle);
    }

    // --- Removal ---

    pub fn remove_triangle(
        &mut self,
        tile_key: TileKey,
        side: TriangleSide,
        depth: DepthKey,
    ) -> Option<FlattenedTriangleLair> {
        let map = match side {
            TriangleSide::Left => &mut self.left_triangle,
            TriangleSide::Right => &mut self.right_triangle,
        };
        let entry = map.get_mut(&tile_key)?;
        let removed = entry.remove(&Reverse(depth));
        if entry.is_empty() {
            map.remove(&tile_key);
        }
        removed
    }

    // --- Construction for rendering ---

    /// Collects triangles for one side at this tile, from highest depth
    /// downward, capped at camera_depth, stopping once a solid is hit
    /// (inclusive of the solid triangle).
    fn collect_side<'a>(
        map: &'a HashMap<TileKey, BTreeMap<Reverse<DepthKey>, FlattenedTriangleLair>>,
        tile_key: TileKey,
        camera_depth: DepthKey,
    ) -> Vec<&'a FlattenedTriangleLair> {
        let Some(tree) = map.get(&tile_key) else {
            return Vec::new();
        };

        // Reverse means natural ascending order = descending depth.
        // We want depths <= camera_depth, which in Reverse-space is
        // Reverse(depth) >= Reverse(camera_depth), i.e. range from
        // Reverse(camera_depth)..
        let mut out = Vec::new();
        for (_, tri) in tree.range(Reverse(camera_depth)..) {
            out.push(tri);
            if tri.has_struck_solid {
                break;
            }
        }
        out
    }

    pub fn collect_for_rendering(
        &self,
        tile_key: TileKey,
        camera_depth: DepthKey,
    ) -> RenderTriangles {
        RenderTriangles {
            left: Self::collect_side(&self.left_triangle, tile_key, camera_depth),
            right: Self::collect_side(&self.right_triangle, tile_key, camera_depth),
        }
    }
}

pub struct RenderTriangles<'a> {
    pub left: Vec<&'a FlattenedTriangleLair>,
    pub right: Vec<&'a FlattenedTriangleLair>,
}