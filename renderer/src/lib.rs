pub mod window;
pub mod vulkan;
pub mod importer;
pub mod model;
pub mod shaders;

pub type ResultError = String;
pub type RenderResult<T> = Result<T, ResultError>;
