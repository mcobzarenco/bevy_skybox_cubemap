// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Provides cubemap-based skyboxes for Bevy.
//!
//! # Overview
//!
//! This crate provides a material type, [`SkyboxMaterial`], and bundle, [`SkyboxBundle`], which
//! make it easy to add a skybox to a scene. Skyboxes are implemented as normal entities using a
//! special shader to ensure they always appear around the camera and behind all other objects in
//! the scene.
//!
//! # Basic usage
//!
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy_skybox_cubemap::{SkyboxBundle, SkyboxMaterial, SkyboxPlugin, SkyboxTextureConversion};
//! // Install the skybox plugin:
//! App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(SkyboxPlugin)
//!     .add_startup_system(setup.system());
//!
//! // Configure the skybox.
//! fn setup(
//!    mut commands: Commands,
//!    asset_server: Res<AssetServer>,
//!    mut skyboxes: ResMut<Assets<SkyboxMaterial>>,
//!    mut skybox_conversion: ResMut<SkyboxTextureConversion>,
//! ) {
//!     // Load a texture to use as the skybox.
//!     let skybox_texture = asset_server.load("labeled_skybox.png");
//!     // Convert a flat image the 6 faces above one another into a 6-layer array texture that's
//!     // appropriate for skybox use.
//!     skybox_conversion.make_array(skybox_texture.clone());
//!     // Spawn a skybox entity.
//!     commands.spawn_bundle(SkyboxBundle::new(
//!         skyboxes.add(SkyboxMaterial::from_texture(skybox_texture)),
//!     ));
//! }
//! ```
//!
//! See below for details on the required texture format.
//!
//! Skyboxes are more or less normal entities. Normal Bevy features like render layers and render
//! pass selection should work on them, so it should be possible to have different skyboxes in
//! different cameras using render layers.
//!
//! The skybox is implemented almost entirely in shader code, so aside from the initial texture
//! conversion (which you can do yourself if you prefer), there's no need for additional cameras or
//! marker components or complicated transform setup on the camera or skybox. The shader will ensure
//! that the skybox is always drawn behind all other entities and that the position of both the
//! camera and skybox have no effect.
//!
//! In case you want your skybox to have a different orientation, the rotation compoenent of the skybox's
//! transform *is* respected.
//!
//! # Texture Layout
//!
//! In order to use a Skybox, you need a properly formatted Skybox texture. Appropriate textures for
//! `SkyboxMaterial` should have 6 identically sized square layers which make up the 6 faces of the
//! Skybox. A helper is provided to convert a single-layer `N x 6N` image into a 6 layer image
//! appropriate for a skybox.
//!
//! This is the net of the cube that the orientation of the faces is based on. It is *not* the
//! texture layout that is actually used for rendering.
//!
//! <img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/expected_net.png" />
//!
//! |           | Top (+Y)    |            |           |
//! |-----------|-------------|------------|-----------|
//! | Left (-X) | Front (-Z)  | Right (+X) | Back (+Z) |
//! |           | Bottom (-Y) |            |           |
//!
//! For rendering, the faces are used as separate layers of an array texture in this order:
//!
//! * Right (+X)
//! * Left (-X)
//! * Top (+Y)
//! * Bottom (-Y)
//! * Back (+Z)
//! * Front (-Z)
//!
//! Currently the easiest way to create an image with the appropriate layers is to rearrange the
//! sections of the cube net into a single vertical image in the required order, then when you load
//! the image, send it to [`SkyboxTextureConversion`], which will use
//! [`Texture::reinterpret_stacked_2d_as_array`] to convert it to a 6 layer array once it is loaded.
//!
//! Here is the above net rearranged into the correct order for a skybox texture:
//!
//! <img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/array_format.png" />
//!
//! When converting from a net or a collection of images representing the faces of the skybox, pay
//! attention to their orientation relative to the canonical net above. If you have a net with a
//! differnt face connected to the top and bottom, the easiest thing to do is to simply interpret
//! whatever face matches the top and bottom as the "front" when rearranging the faces into the
//! vertical array format.
//!
//! For example, if the top and bottom branch off of the third square instead of the second, you
//! could interpret the net this way:
//!
//! <img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/shifted_net.png" />
//!
//! You would then rearrange from this net to the same vertical layout as above.
//!
//! Alternately, if want a specific face to be used as the "front" and that face isn't the one that
//! matches the orientation of the top and bottom, you could instead rotate the top and bottom when
//! building the stacked array texture. However, since you can also rotate the skybox using the
//! skybox entity's transform, that's probably not necessary.
//!
//! # Maintenance of this Crate
//!
//! Bevy is a cool project and I am excited for it to succeed. However, I don't necessarily have
//! time to always keep this crate up to date with the latest versions of Bevy, especially if it
//! gets relatively low usage.
//!
//! That said, I *will* respond to pull requests and will release new versions based on pull
//! requests to update to newer versions of Bevy. Creating a pull request is preferable to opening
//! an issue asking me to update, because I can more easily spare the time to merge a pull request
//! than to do all the necessary updates myself, but if I do get issues asking me to update to new
//! versions of Bevy, I will respond to them on a best-effort basis.
//!
//! I will create releases targeting the latest published version of Bevy, not `main`. If you are
//! working on main, and need to modify this crate to work with the latest `HEAD`, I recommend
//! forking and then sending me a pull request once Bevy publishes an updated version.
//!
//! In terms of features, I would consider this project largely feature-complete as-is. Texture
//! packaging seems to be outside the scope of features supported by Bevy, so I'm not going to add
//! tools to automate building textures for skyboxes. This crate also isn't intended to support any
//! kind of dynamic skyboxes, so there doesn't seem to be much more that needs to be done besides
//! keeping up with latest versions of Bevy. However, if you have any ideas for new features or API
//! changes, I'm happy to hear them.
//!
//! # Disclaimer
//!
//! This is not an officially supported Google product.

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, NotShadowCaster, NotShadowReceiver, SpecializedMaterial},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferInitDescriptor,
            BufferUsages, Face, RenderPipelineDescriptor, ShaderStage, ShaderStages, *,
        },
        renderer::RenderDevice,
        view::visibility::NoFrustumCulling,
    },
};

/// Configures the skybox render pipeline and support for [`SkyboxMaterial`]. Also sets up the system for [`
pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        // Add the Skybox shaders
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            SKYBOX_VERTEX_SHADER_HANDLE,
            Shader::from_glsl(include_str!("skybox.vert"), ShaderStage::Vertex),
        );
        shaders.set_untracked(
            SKYBOX_FRAGMENT_SHADER_HANDLE,
            Shader::from_glsl(include_str!("skybox.frag"), ShaderStage::Fragment),
        );

        // Add the Skybox mesh
        let mut meshes = app.world.get_resource_mut::<Assets<Mesh>>().unwrap();
        // Skybox mesh needs to be large enough not to get caught in the camera's near-clip plane (but
        // can otherwise be any value).
        meshes.set_untracked(SKYBOX_MESH_HANDLE, Mesh::from(shape::Cube { size: 1.0 }));

        app.add_plugin(MaterialPlugin::<SkyboxMaterial>::default())
            .add_system(convert_skyboxes)
            .init_resource::<SkyboxTextureConversion>();
    }
}

/// Bundle for spawning Skybox entities. Note that you should be able to use defaults for everything
/// besides `material`. The only other field you may want to touch is `transform` which can be used
/// to rotate the skybox if desired. Translations applied to skyboxes are ignored.
///
/// When inserting a skybox bundle, you should generally use `..Default::default()` for every
/// property except the `material` and occasionally `transform` (if you want to rotate the skybox
/// from its default orientation).
///
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_skybox_cubemap::{SkyboxBundle, SkyboxMaterial, SkyboxPlugin, SkyboxTextureConversion};
/// # App::new()
/// #     .add_startup_system(setup.system());
/// # fn setup(
/// #     mut commands: Commands,
/// #     asset_server: Res<AssetServer>,
/// #     mut meshes: ResMut<Assets<Mesh>>,
/// #     mut materials: ResMut<Assets<StandardMaterial>>,
/// #     mut skyboxes: ResMut<Assets<SkyboxMaterial>>,
/// #     mut skybox_conversion: ResMut<SkyboxTextureConversion>,
/// # ) {
/// # let skybox_texture = asset_server.load("labeled_skybox.png");
/// # skybox_conversion.make_array(skybox_texture.clone());
/// commands.spawn_bundle(SkyboxBundle::new(
///     skyboxes.add(SkyboxMaterial::from_texture(skybox_texture)),
/// ));
/// # }
/// ```
#[derive(Bundle)]
pub struct SkyboxBundle {
    /// Material to use for the skybox. Defaults to a garish pink. In most usage this should be the
    /// only field you need to set.
    pub material: Handle<SkyboxMaterial>,
    /// Mesh to use for the skybox. Defaults to [`SKYBOX_MESH_HANDLE`], which is a unit cube. You
    /// shouldn't ever need to use any other mesh. Because of how cubemap sampling works, probably
    /// any mesh that completely surrounds the camera would work equally well, but only the unit
    /// cube is officially supported by this crate.
    pub mesh: Handle<Mesh>,
    /// User indication of whether the skybox is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
    pub no_frustum_culling: NoFrustumCulling,
    pub not_shadow_caster: NotShadowCaster,
    pub not_shadow_receiver: NotShadowReceiver,
    /// Transform can be used to manipulate the rotation of the skybox.
    pub transform: Transform,
    /// Transforms get computed into global transforms used for drawing based on parenting. Note
    /// that it doesn't make much sense to add a skybox as a child of any other entity; it should
    /// usually be freestanding.
    pub global_transform: GlobalTransform,
}

impl SkyboxBundle {
    /// Convenience constructor for [`SkyboxBundle`]. Sets the material and uses defaults for
    /// everything else. In most use cases you should only need to set the material.
    pub fn new(material: Handle<SkyboxMaterial>) -> Self {
        Self {
            material,
            ..Default::default()
        }
    }
}

impl Default for SkyboxBundle {
    fn default() -> Self {
        Self {
            material: Default::default(),
            mesh: SKYBOX_MESH_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            no_frustum_culling: NoFrustumCulling,
            not_shadow_caster: NotShadowCaster,
            not_shadow_receiver: NotShadowReceiver,
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// Material for a Skybox. Consists of a base color and an optional 6-sided array-texture.
///
/// When rendering, the color from the texure is multiplied by the base color. This can be used to
/// tint the skybox. When creating a new material, the default color is [`Color::WHITE`] which will
/// have no effect on the texture color.
///
/// It is also possible to use a skybox texture with only a [`Color`]. One reason you might want to
/// do this is that (at time of writing) Bevy does not seem to antialias against the window
/// [`ClearColor`] properly, instead antialiasing with white for objects that have not other 3d
/// object behind them. This leads to white borders around antialiased object that overlap the
/// window clear color. To avoid this, you could spawn a skybox using only a color. Since the skybox
/// is a 3d rendered object, antialiasing against it works properly.
///
/// Skyboxes should generally be spawned using [`SkyboxBundle`], and you can see that type for info
/// on what components are used with this material.
#[derive(Debug, Clone, TypeUuid)]
// UUID5 generated by first creating a URL-namespaced UUID5 for
// "https://github.com/google/bevy_skybox_cubemap" (24291f52-ea01-574a-b6ae-3d8182f6086b) then using
// that as the namespace with `bevy_skybox_cubemap::SkyboxMaterial` as the name.
#[uuid = "fca7708e-57bb-5a81-977f-95b0e5202de0"]
pub struct SkyboxMaterial {
    /// Base color of the skybox. Multiplied with the color from the texture if a texture is
    /// supplied, otherwise used by itself as the skybox color.
    pub color: Color,
    /// Texture to use for the skybox. This must be a an aray texture with 6 layers which are all
    /// square and the same size. See [the crate overview](crate) for details on the required layer
    /// order and how to get a texture in this format.
    pub texture: Option<Handle<Image>>,
}

#[derive(Clone)]
pub struct GpuSkyboxMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl SkyboxMaterial {
    /// Creates a `SkyboxMaterial` with just a texture. The color will be set to [`Color::WHITE`] to
    /// avoid tinting the texture.
    pub fn from_texture(texture: Handle<Image>) -> Self {
        Self {
            texture: Some(texture),
            ..Default::default()
        }
    }

    /// Creates a `SkyboxMaterial` with only a color. This could be used in place of [`ClearColor`]
    /// if `ClearColor` is giving you issues with antialiasing. Otherwise it's not all that useful.
    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
}

impl Default for SkyboxMaterial {
    /// Creates a new skybox material with color set to white and no texture.
    fn default() -> Self {
        Self {
            // Set the default color to white, so when using with a texture the color doesn't impact
            // the texture color.
            color: Color::WHITE,
            texture: None,
        }
    }
}

impl RenderAsset for SkyboxMaterial {
    type ExtractedAsset = SkyboxMaterial;
    type PreparedAsset = GpuSkyboxMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<MaterialPipeline<Self>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, material_pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&material.color.as_linear_rgba_f32());
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: color.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let (base_color_texture_view, base_color_sampler) = if let Some(result) = material_pipeline
            .mesh_pipeline
            .get_image_texture(gpu_images, &material.texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(base_color_texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(base_color_sampler),
                },
            ],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuSkyboxMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl SpecializedMaterial for SkyboxMaterial {
    type Key = ();

    fn key(_: &<SkyboxMaterial as RenderAsset>::PreparedAsset) -> Self::Key {}

    fn specialize(_: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        descriptor.primitive.cull_mode = Some(Face::Front);
    }

    fn vertex_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(SKYBOX_VERTEX_SHADER_HANDLE.typed::<Shader>())
    }

    fn fragment_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(SKYBOX_FRAGMENT_SHADER_HANDLE.typed::<Shader>())
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                    },
                    count: None,
                },
                // Texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2Array,
                    },
                    count: None,
                },
                // Texture Sampler
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        })
    }
}

/// Resource to help with converting skyboxes stored as vertically stacked images as described in
/// the [crate] documentation into array textures in the correct format for use in a
/// [`SkyboxMaterial`].
///
/// The [`SkyboxPlugin`] will add this resource and install an associated system which handles the
/// actual texture conversion. Conversion is performed using
/// [`Texture::reinterpret_stacked_2d_as_array`]. If you prefer, you are free to handle converting
/// textures yourself, or use a texture format + loader which can load array textures directly.
#[derive(Default)]
pub struct SkyboxTextureConversion {
    /// List of texture handles that should be skyboxes.
    handles: Vec<Handle<Image>>,
}

impl SkyboxTextureConversion {
    /// Takes a handle to a texture whose dimensions are `N` wide by `6*N` high, waits for it to load,
    /// and then reinterprets that texture as an array of 6 textures suitable or a skybox. This is
    /// useful if your skybox texture is not in a format that has layers. This should only be done
    /// once per testure, and will panic if the texture has already be reinterpreted.
    pub fn make_array(&mut self, handle: Handle<Image>) {
        self.handles.push(handle);
    }
}

/// System to handle reinterpreting an Nx6N vertical texture stack as an array of textures suitable
/// for a skybox.
fn convert_skyboxes(
    mut conversions: ResMut<SkyboxTextureConversion>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut index = 0;
    while let Some(handle) = conversions.handles.get(index) {
        // Check each texture in the pending queue to see if it is loaded yet.
        let (handle, texture) = match textures.get_mut(handle) {
            // If it's loaded, take it out of the queue.
            Some(texture) => (conversions.handles.remove(index), texture),
            None => {
                index += 1;
                continue;
            }
        };

        info!(
            "Reinterpreting as Skybox Texture {:?}: len: {}",
            handle,
            texture.data.len(),
        );
        texture.reinterpret_stacked_2d_as_array(6);
    }
}

/// Handle to use to reference the skybox pipeline.
const SKYBOX_VERTEX_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 16037920303847147810);
const SKYBOX_FRAGMENT_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7123103161671906218);

/// Handle to use to reference the skybox mesh.
const SKYBOX_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 7423141153313829192);
