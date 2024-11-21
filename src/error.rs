use derive_more::Display;
use std::{error::Error as StdError, fmt::Debug};

#[derive(Debug, Display)]
pub enum Error<E>
where
    E: 'static + Debug + Send + Sync,
{
    #[display(fmt = "{}", _0)]
    Delivery(actix::MailboxError),

    #[display(fmt = "{}", _0)]
    Timeout(r2d2::Error),

    #[display(fmt = "{:?}", _0)]
    Execute(E),
}

impl<E> StdError for Error<E> where E: 'static + Debug + Send + Sync {}

#[cfg(feature = "actix-web")]
impl<E> actix_web::ResponseError for Error<E> where E: 'static + Debug + Send + Sync {}

#[cfg(feature = "failure")]
impl Error<failure::Error> {
    pub fn downcast<T: failure::Fail>(self) -> Result<T, Self> {
        match self {
            Error::Execute(err) => err.downcast().map_err(Error::Execute),
            err => Err(err),
        }
    }

    pub fn downcast_ref<T: failure::Fail>(&self) -> Option<&T> {
        match self {
            Error::Execute(err) => err.downcast_ref(),
            _ => None,
        }
    }

    pub fn downcast_mut<T: failure::Fail>(&mut self) -> Option<&mut T> {
        match self {
            Error::Execute(err) => err.downcast_mut(),
            _ => None,
        }
    }
}
