use bevy::prelude::*;
use rstar::{DefaultParams, PointDistance, RTree, RTreeObject, RTreeParams, AABB};

use crate::{common::EntityPoint3D, rtree::common::RTreeAccess, spatial_access::SpatialAccess};

pub type RTreeAccess3D<TComp, Params = DefaultParams> = RTreeAccess<TComp, EntityPoint3D, Params>;

impl<TComp, Params> SpatialAccess for RTreeAccess3D<TComp, Params>
where
    Params: RTreeParams,
    TComp: Component + Sync + 'static,
{
    /// The component which this tree tracks.
    type TComp = TComp;

    /// Squared distance between 2 Vec3s.
    ///
    /// For 2d trees this will discard the z component of the Vec3.
    fn distance_squared(&self, loc_a: Vec3, loc_b: Vec3) -> f32 {
        loc_a.distance_squared(loc_b)
    }

    /// Get the nearest neighbour to a position.
    fn nearest_neighbour(&self, loc: Vec3) -> Option<(Vec3, Entity)> {
        let res = self.tree.nearest_neighbor(&[loc.x, loc.y, loc.z]);
        res.map(|point| (point.vec, point.entity))
    }

    /// Get the `k` neighbours to `loc`
    ///
    /// If `loc` is the location of a tracked entity, you might want to skip the first.
    fn k_nearest_neighbour(&self, loc: Vec3, k: usize) -> Vec<(Vec3, Entity)> {
        return self
            .tree
            .nearest_neighbor_iter(&[loc.x, loc.y, loc.z])
            .take(k)
            .map(|e| (e.vec, e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }

    /// Get all entities within a certain distance (radius) of `loc`
    fn within_distance(&self, loc: Vec3, distance: f32) -> Vec<(Vec3, Entity)> {
        return self
            .tree
            .locate_within_distance([loc.x, loc.y, loc.z], distance.powi(2))
            .map(|e| (e.vec, e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }

    /// Recreates the tree with the provided entity locations/coordinates.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn recreate(&mut self, all: Vec<(Vec3, Entity)>) {
        let tree: RTree<EntityPoint3D, Params> =
            RTree::bulk_load_with_params(all.iter().map(|e| e.into()).collect());
        self.tree = tree;
    }

    /// Adds a point to the tree.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn add_point(&mut self, point: (Vec3, Entity)) {
        self.tree.insert(point.into())
    }

    /// Adds a point to the tree.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn remove_point(&mut self, point: (Vec3, Entity)) -> bool {
        self.tree.remove(&point.into()).is_some()
    }

    /// Removes a point from the tree.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn remove_entity(&mut self, entity: Entity) -> bool {
        self.tree.remove(&entity.into()).is_some()
    }

    /// Size of the tree
    fn size(&self) -> usize {
        self.tree.size()
    }

    /// Get the distance after which a entity is updated in the tree
    fn get_min_dist(&self) -> f32 {
        self.min_moved
    }

    /// Get the amount of entities which moved per frame after which the tree is fully recreated instead of updated.
    fn get_recreate_after(&self) -> usize {
        self.recreate_after
    }
}

impl RTreeObject for EntityPoint3D {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.vec.into())
    }
}

// TODO: currently somewhat duplicating the SpatialAccess distance calculation - how to resolve?
impl PointDistance for EntityPoint3D {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        self.vec.distance_squared(Vec3::from_slice(point))
    }
}
