use nom::{error::VerboseError, IResult};

pub type Result<I, O> = IResult<I, O, VerboseError<I>>;
