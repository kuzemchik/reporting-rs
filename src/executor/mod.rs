pub mod planner;
pub mod query;
pub mod templates;

#[macro_export]
macro_rules! rc {
    ($str:literal) => {
        Rc::from($str)
    };
}
