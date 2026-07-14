mod health;

use crate::{
    error::{PatchwiseError, Result},
    notify,
};

use nvim_oxi::{
    Function,
    api::{self, types::CommandArgs},
};
use strum::{EnumIter, IntoEnumIterator};

type Handler = fn(CommandArgs) -> Result<()>;

#[derive(Debug, Clone, Copy, EnumIter)]
enum Command {
    Health,
}

impl Command {
    const fn name(self) -> &'static str {
        match self {
            Self::Health => "PatchwiseHealth",
        }
    }

    const fn handler(self) -> Handler {
        match self {
            Self::Health => health::run,
        }
    }

    fn register(self) -> Result<()> {
        let name = self.name();
        let handler = self.handler();

        let callback = Function::from_fn(move |args: CommandArgs| {
            if let Err(error) = handler(args) {
                notify::error(&format!("{name}: {error}"));
            }
        });

        Self::create_oxi_user_command(name, callback)
    }

    fn create_oxi_user_command(
        name: &'static str,
        callback: Function<CommandArgs, ()>,
    ) -> Result<()> {
        api::create_user_command(name, callback, &Default::default())
            .map_err(|source| PatchwiseError::CommandRegistration { name, source })
    }
}

pub fn register_all() -> Result<()> {
    for command in Command::iter() {
        command.register()?;
    }

    Ok(())
}
