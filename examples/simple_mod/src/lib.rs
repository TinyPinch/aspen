use bevy_app::{App, Update};
use canopy::prelude::*;
use clap::Parser;
use tracing::info;

pub struct SimpleMod;

#[derive(Parser)]
pub struct Arguments;

impl CanopyMod for SimpleMod {
    type Arguments = Arguments;

    fn initialize(_: &Self::Arguments) -> Self {
        Self
    }

    fn build(&self, app: &mut App) -> canopy::Result<()> {
        app.canopy_add_systems(Update, system)?;

        Ok(())
    }
}

fn system() {
    info!("hooked in");
}

canopy_mod!(SimpleMod);
