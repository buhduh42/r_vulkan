pub mod window;
pub mod vulkan;
pub mod importer;
pub mod model;

pub type ResultError = String;
pub type RenderResult<T> = Result<T, ResultError>;
