use crate::math::{Point, Real, Vector};

#[cfg_attr(feature = "cuda", derive(cust_core::DeviceCopy))]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RigidParticle {
    pub position: Point<Real>,
    pub collider_index: u32, // Todo: consider packing both indices into a single u32
    pub triangle_index: u32,
    pub color_index: u32,
}
