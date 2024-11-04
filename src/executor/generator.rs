use crate::models::models::{Column, Report};

trait Visitor<I, O> {
    fn visit(&self, input: I) -> O;
}

trait Visitable {
    fn visit<'a, O, Vis: Visitor<&'a Self, O>>(&'a self, visitor: &'a Vis) -> O {
        visitor.visit(&self)
    }
}

impl Visitable for Column {}
impl Visitable for Report {}

// impl Visitor<String, Result<Column, Error>> for ConversionVisitor {
//     fn visit(&self, input: String) -> Result<Column, Error> {
//         self.datasource
//             .columns
//             .iter()
//             .find(|c| c.column_id.to_string() == input)
//             .cloned()
//             .ok_or(ColumnNotFound(input.clone()))
//     }
// }
