#[cfg(feature = "dim2")]
pub mod fluid_2d;
#[cfg(feature = "dim3")]
pub mod fluid_3d;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d_elasticity;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d_surface_tension_akinci;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d_surface_tension_he;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d_surface_tension_wcsph;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d_viscosity_artificial;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d_viscosity_dfsph;
#[cfg(feature = "dim2")]
pub mod fluid_effect_2d_viscosity_xsph;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d_elasticity;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d_surface_tension_akinci;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d_surface_tension_he;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d_surface_tension_wcsph;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d_viscosity_artificial;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d_viscosity_dfsph;
#[cfg(feature = "dim3")]
pub mod fluid_effect_3d_viscosity_xsph;
pub mod fluid_impl;
pub mod rapier_fluid;
pub mod types;
