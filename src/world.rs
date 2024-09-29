use std::{any::Any, ops::{Deref, DerefMut}};

use bevy_ecs::{system::Resource, world::{Mut, World}};

use crate::{CanopyError, DUMP};

pub trait CanopyWorldExt {
    fn canopy_resource<R: Resource>(&self) -> crate::Result<Option<&R>>;
    fn canopy_resource_mut<'w, R: Resource>(&'w mut self) -> crate::Result<Option<Mut<'w, R>>>;
}

impl CanopyWorldExt for World {
    fn canopy_resource<R: Resource>(&self) -> crate::Result<Option<&R>> {
        let type_id = DUMP.types().type_id_of::<R>().ok_or(CanopyError::TypeNotFound)?;
        let Some(component_id) = self.components().get_resource_id(type_id) else {
            return Ok(None);
        };

        let Some(ptr) = self.get_resource_by_id(component_id) else {
            return Ok(None);
        };

        Ok(Some(unsafe { ptr.deref() }))
    }

    fn canopy_resource_mut<'w, R: Resource>(&'w mut self) -> crate::Result<Option<Mut<'w, R>>> {
        let type_id = DUMP.types().type_id_of::<R>().ok_or(CanopyError::TypeNotFound)?;
        let Some(component_id) = self.components().get_resource_id(type_id) else {
            return Ok(None);
        };

        let Some(ptr) = self.get_resource_mut_by_id(component_id) else {
            return Ok(None);
        };

        Ok(Some(unsafe { ptr.with_type() }))
    }
}
