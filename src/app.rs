use bevy_app::App;
use bevy_ecs::schedule::{IntoSystemConfigs, ScheduleLabel, Schedules};
use tracing::error;

use crate::world::CanopyWorldExt;

pub trait CanopyAppExt {
    fn canopy_add_systems<M>(&mut self, label: impl ScheduleLabel, systems: impl IntoSystemConfigs<M>) -> crate::Result<()>;
}

impl CanopyAppExt for App {
    fn canopy_add_systems<M>(&mut self, label: impl ScheduleLabel, systems: impl IntoSystemConfigs<M>) -> crate::Result<()> {
        let world = self.world_mut();

        let Some(mut schedules) = world.canopy_resource_mut::<Schedules>()? else {
            error!("Could not get schedules resoruce");
            return Ok(());
        };

        let Some(schedule) = schedules
            .iter_mut()
            // TODO: Fix this terrible mess
            .find_map(|(l, s)| if format!("{l:?}") == format!("{label:?}") { Some(s) } else { None }) else {
            error!("Could not find schedule: {label:?}");
            return Ok(());
        };

        schedule.add_systems(systems);

        Ok(())
    }
}
