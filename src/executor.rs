use actix::{Actor, Handler, Message, SyncContext};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection,
};
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Executor<C: 'static>(pub Pool<ConnectionManager<C>>)
where
    C: Connection;

impl<C> Actor for Executor<C>
where
    C: Connection,
{
    type Context = SyncContext<Self>;
}

pub struct Execute<F, C, R, E>(pub F, pub PhantomData<(C, R)>)
where
    R: 'static + Send,
    E: 'static + Debug + Send + Sync,
    C: Connection,
    F: FnOnce(&C) -> Result<R, E>;

impl<F, C, R, E> Message for Execute<F, C, R, E>
where
    R: Send,
    E: Debug + Send + Sync,
    C: Connection,
    F: FnOnce(&C) -> Result<R, E>,
{
    type Result = Result<Result<R, E>, r2d2::Error>;
}

impl<F, C, R, E> Handler<Execute<F, C, R, E>> for Executor<C>
where
    R: Send,
    E: Debug + Send + Sync,
    C: Connection,
    F: FnOnce(&C) -> Result<R, E>,
{
    type Result = Result<Result<R, E>, r2d2::Error>;

    fn handle(&mut self, msg: Execute<F, C, R, E>, _: &mut Self::Context) -> Self::Result {
        let conn = match self.0.get() {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };

        Ok((msg.0)(&*conn))
    }
}
