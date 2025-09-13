// src/mobs.rs

#[path = "mobs/boss.rs"]
pub mod boss;

#[path = "mobs/member.rs"]
pub mod member;

#[path = "mobs/mob.rs"]
pub mod mob;

// Re-export for external access
pub use boss::Boss;
pub use member::{Member, Role};
pub use mob::Mob;
