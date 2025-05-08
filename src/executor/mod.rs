pub mod planner;
pub mod query;


#[macro_export]
macro_rules! rc {
    ($str:literal) => {
        Rc::from($str)
    };
}
