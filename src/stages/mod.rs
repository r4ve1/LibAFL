pub mod mutational;

use crate::corpus::Testcase;
use crate::engines::Evaluator;
use crate::inputs::Input;
use crate::AflError;

use std::cell::RefCell;
use std::rc::Rc;

pub trait HasEvaluator<I>
where
    I: Input,
{
    type E: Evaluator<I>;

    fn eval(&self) -> &Self::E;

    fn eval_mut(&mut self) -> &mut Self::E;
}

pub trait Stage<'a, I>: HasEvaluator<I>
where
    I: Input,
{
    /// Run the stage
    fn perform(&mut self, entry: Rc<RefCell<Testcase<I>>>) -> Result<(), AflError>;
}
