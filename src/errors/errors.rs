use actix_web::error;
use deadpool_postgres::PoolError;

#[derive(failure::Fail, Debug)]
pub enum Error {
    #[fail(display = "An internal error occurred. Please try again later.")]
    PoolError(PoolError),
    #[fail(display = "An internal error occurred. Please try again later.")]
    IOError(std::io::Error),
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Self::PoolError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl error::ResponseError for Error {}
