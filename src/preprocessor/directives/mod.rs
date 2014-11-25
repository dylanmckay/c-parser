
pub use self::define::Define;

pub mod define;

/// A preprocessor directive.
#[deriving(Show)]
pub enum Directive
{
    Define(Define),
}
