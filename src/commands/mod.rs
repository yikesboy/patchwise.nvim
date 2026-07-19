mod health;
mod replace;
mod selection;

use crate::{
    error::{PatchwiseError, Result},
    notify,
};

use nvim_oxi::{
    Function,
    api::{
        self,
        opts::CreateCommandOpts,
        types::{CommandArgs, CommandRange},
    },
};
use strum::{EnumIter, IntoEnumIterator};

type Handler = fn(CommandArgs) -> Result<()>;

#[derive(Debug, Clone, Copy, EnumIter)]
enum Command {
    Health,
    Selection,
    Replace,
}

impl Command {
    const fn name(self) -> &'static str {
        match self {
            Self::Health => "PatchwiseHealth",
            Self::Selection => "PatchwiseSelection",
            Self::Replace => "PatchwiseReplace",
        }
    }

    const fn handler(self) -> Handler {
        match self {
            Self::Health => health::run,
            Self::Selection => selection::run,
            Self::Replace => replace::run,
        }
    }

    fn options(self) -> CreateCommandOpts {
        match self {
            Self::Health => CreateCommandOpts::default(),
            Self::Selection | Self::Replace => CreateCommandOpts::builder()
                .range(CommandRange::CurrentLine)
                .build(),
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

        let opts = self.options();

        Self::create_oxi_user_command(name, callback, opts)
    }

    fn create_oxi_user_command(
        name: &'static str,
        callback: Function<CommandArgs, ()>,
        options: CreateCommandOpts,
    ) -> Result<()> {
        api::create_user_command(name, callback, &options)
            .map_err(|source| PatchwiseError::CommandRegistration { name, source })
    }
}

pub fn register_all() -> Result<()> {
    for command in Command::iter() {
        command.register()?;
    }

    Ok(())
}
