pub mod queue_searcher;
pub mod recursive_searcher;

pub use self::queue_searcher::QueueSearcher;
pub use self::recursive_searcher::RecursiveSearcher;
use std::collections::HashMap;

const LIMIT: usize = 24;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Node {
    pub text: String,
    pub previous: Option<String>,
}

pub trait Searcher {
    fn search(
        &self,
        memo: &mut HashMap<Node, (f64, Vec<String>)>,
        text: &str,
        previous: Option<&str>,
    ) -> (f64, Vec<String>);
}
