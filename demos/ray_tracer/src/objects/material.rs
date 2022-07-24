use super::MaterialType;
use wrend::Vec3;

#[derive(Clone, PartialEq, Debug, Copy, PartialOrd, Default)]
pub struct Material {
    pub material_type: MaterialType,
    pub albedo: Vec3,          // or "reflectance"
    pub fuzz: f32,             // used for duller metals
    pub refraction_index: f32, // used for glass
}
