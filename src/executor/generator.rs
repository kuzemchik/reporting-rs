use crate::domain::models::{Column, Datasource, Report};
use crate::executor::query::SqlAst;

enum Error {}
trait Visitor<I, O> {
    fn visit(&self, input: I) -> O;
}

trait Visitable {
    fn accept<'a, O, Vis: Visitor<&'a Self, O>>(
        &'a self,
        visitor: &'a Vis,
    ) -> O {
        visitor.visit(&self)
    }
}

impl Visitable for Column {}
impl Visitable for Report {}

struct QueryPlanner {
    datasource: Datasource,
}
// impl Visitor<Report, Result<SqlAst, Error>> for QueryPlanner {
//     fn visit(&self, input: Report) -> Result<SqlAst, Error> {
//         let sqlAst =
//             input.columns
//     }
// }
