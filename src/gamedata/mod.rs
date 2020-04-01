//! Contain everything related to the internal state of the actual scene

mod scene;
pub use scene::Scene;

mod update;
pub use update::Update;

mod vectors;
pub use vectors::Vec3;

mod characters;
pub use characters::Chara;

mod various;
pub use various::Time;

mod lock;
pub use lock::Lock;

mod color;
pub use color::Color;

mod screen;
pub use screen::Screen;
