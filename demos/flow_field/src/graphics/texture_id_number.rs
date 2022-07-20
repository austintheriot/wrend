/// Allows associating a TextureId with it's texture location binding
pub trait TextureIdNumber {
    fn num(&self) -> u32;
}