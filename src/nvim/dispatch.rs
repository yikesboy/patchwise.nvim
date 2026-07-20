use std::sync::{
    OnceLock,
    mpsc::{self, Sender},
};

use nvim_oxi::libuv::AsyncHandle;

use crate::error::{PatchwiseError, Result};

type Task = Box<dyn FnOnce() + Send + 'static>;

struct Dispatcher {
    sender: Sender<Task>,
    handle: AsyncHandle,
}

static DISPATCHER: OnceLock<Dispatcher> = OnceLock::new();

pub fn init() -> Result<()> {
    if DISPATCHER.get().is_some() {
        return Ok(());
    }

    let (sender, receiver) = mpsc::channel::<Task>();

    let handle = AsyncHandle::new(move || {
        while let Ok(task) = receiver.try_recv() {
            task();
        }
    })
    .map_err(PatchwiseError::DispatchInitiallization)?;

    let _ = DISPATCHER.set(Dispatcher { sender, handle });

    Ok(())
}

pub fn dispatch<F>(task: F) -> Result<()>
where
    F: FnOnce() + Send + 'static,
{
    let dispatcher = DISPATCHER
        .get()
        .ok_or(PatchwiseError::DispatchNotInitialized)?;

    dispatcher
        .sender
        .send(Box::new(task))
        .map_err(|_| PatchwiseError::DispatchClosed)?;

    dispatcher
        .handle
        .send()
        .map_err(PatchwiseError::DispatchWake)
}
