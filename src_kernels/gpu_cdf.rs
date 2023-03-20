use crate::cuda::AtomicInt;
use cuda_std::GpuFloat;
use sparkl_core::math::{Real, Vector};

const FREE: u32 = 0;
const LOCKED: u32 = 1;

#[cfg_attr(not(target_os = "cuda"), derive(cust::DeviceCopy))]
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct CdfColor(pub u32);

impl CdfColor {
    pub fn new(affinity: u32, tag: u32, collider_index: u32) -> Self {
        Self((affinity << collider_index) | (tag << (collider_index + 16)))
    }

    pub fn affinity(&self, collider_index: u32) -> u32 {
        1 & (self.0 >> collider_index)
    }

    pub fn tag(&self, collider_index: u32) -> u32 {
        1 & (self.0 >> (collider_index + 16))
    }

    pub fn set_affinity(&mut self, collider_index: u32) {
        self.0 |= 1 << collider_index;
    }

    pub fn change_tag(&mut self, collider_index: u32, value: u32) {
        // sets the bit at collider_index + 16 to the value
        let offset = collider_index + 16;
        self.0 = self.0 & !(1 << offset) | (value << offset);
    }

    pub fn affinities(&self) -> u32 {
        self.0 & 0xFFFF
    }
}

#[cfg_attr(not(target_os = "cuda"), derive(cust::DeviceCopy))]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GridCdfData {
    // The unsigned distance to the closest collider.
    pub unsigned_distance: f32,
    // The affinity and tag (inside/ outside) information stored for up to 16 colliders.
    pub color: CdfColor,
    lock: u32,
}

impl Default for GridCdfData {
    fn default() -> Self {
        Self {
            unsigned_distance: 0.0,
            color: Default::default(),
            lock: FREE,
        }
    }
}

impl GridCdfData {
    pub fn update(&mut self, signed_distance: Real, collider_index: u32, tid: u32) {
        let unsigned_distance = signed_distance.abs();
        let tag = if signed_distance >= 0.0 { 1 } else { 0 };

        // Todo: fix IllegalAddress error
        unsafe {
            while self.lock.shared_atomic_exch_acq(LOCKED) == LOCKED {}

            self.color.set_affinity(collider_index);

            // only update the tag information if the new distance is smaller than the old one
            if unsigned_distance < self.unsigned_distance {
                self.color.change_tag(collider_index, tag);
                self.unsigned_distance = unsigned_distance;
            }

            self.lock.shared_atomic_exch_rel(FREE);
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct InterpolatedCdfData {
    affinities: u32,
    tags: u32,
    weighted_tags: [f32; 16],
    signed_distance: f32,
    gradient: Vector<Real>,
}

impl InterpolatedCdfData {
    pub fn interpolate_color(&mut self, node_cdf: GridCdfData, weight: Real) {
        let unsigned_distance = node_cdf.unsigned_distance;

        self.affinities |= node_cdf.color.affinities();

        for collider_index in 0..16 {
            let tag = node_cdf.color.tag(collider_index as u32);
            let tag = if tag == 1 { 1.0 } else { -1.0 };

            self.weighted_tags[collider_index] += weight * unsigned_distance * tag;
        }
    }

    pub fn compute_tags(&mut self) {
        for collider_index in 0..16 {
            let weighted_tag = self.weighted_tags[collider_index];
            let tag = if weighted_tag >= 0.0 { 1 } else { 0 };
            self.tags = tag << (collider_index as u32 + 16);
        }
    }

    pub fn interpolate_distance_and_normal(
        &mut self,
        node_cdf: GridCdfData,
        weight: Real,
        inv_d: Real,
        dpt: Vector<Real>,
    ) {
    }
}

pub struct ParticleCdf {
    color: u32,
    unsigned_distance: f32,
    normal: Vector<Real>,
}
