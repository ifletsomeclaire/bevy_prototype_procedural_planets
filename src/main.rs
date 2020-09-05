use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::vec3,
    prelude::*,
    render::camera::PerspectiveProjection,
    render::{
        mesh::shape,
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{asset_shader_defs_system, ShaderDefs, ShaderStage, ShaderStages},
    },
};
use noise::*;

mod wasd_camera;
use wasd_camera::{CameraConfig, CameraMarker};
// mod colored_mesh;

#[derive(RenderResources, ShaderDefs)]
struct StellarMaterial {
    pub basecolor: Color,
    #[shader_def]
    pub texture: Option<Handle<Texture>>,
    pub atmo_radius: f32,
    pub camera_pos: Mat4,
}

#[derive(Default)]
struct AssetHandles {
    handle: Handle<StellarMaterial>,
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_asset::<StellarMaterial>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_resource(CameraConfig::default())
        .add_resource(AssetHandles::default())
        .add_plugin(wasd_camera::WasdCamera)
        .add_startup_system(setup.system())
        .add_system(update_camera_pass_through.system())
        .add_system_to_stage(
            stage::POST_UPDATE,
            asset_shader_defs_system::<StellarMaterial>.system(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StellarMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // let texture_handle = asset_server.load("assets/unscaledFinalPlanet.png").unwrap();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../assets/vert_shader.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../assets/frag_shader.frag"),
        ))),
    }));
    render_graph.add_system_node(
        "stellar_material",
        AssetRenderResourcesNode::<StellarMaterial>::new(true),
    );
    render_graph
        .add_node_edge("stellar_material", base::node::MAIN_PASS)
        .unwrap();

    let material = materials.add(StellarMaterial {
        basecolor: Color::rgb(1.0, 1.0, 1.0),
        texture: None,
        atmo_radius: 55000.0,
        camera_pos: Mat4::from_translation(vec3(0.0, 0.0, 100000.)),
        // texture: Some(texture_handle),
    });
    commands.spawn(LightComponents {
        translation: Translation::new(40000.0, -4.0, 100000.0),
        ..Default::default()
    });
    let mut noise = Vec::new();
    let mut atmo_noise = Vec::new();
    let mut mesh = Mesh::from(shape::Icosphere {
        radius: 50000.,
        subdivisions: 20,
    });
    let mut atmo = Mesh::from(shape::Icosphere {
        radius: 42000.,
        subdivisions: 20
    });

    match mesh.attributes[0].values {
        bevy::render::mesh::VertexAttributeValues::Float(_) => {}
        bevy::render::mesh::VertexAttributeValues::Float2(_) => {}
        bevy::render::mesh::VertexAttributeValues::Float3(ref mut val) => {
            for verts in val {
                let x = noise::RidgedMulti::new();
                let mut n = x.get([verts[0] as f64, verts[1] as f64, verts[2] as f64]);
                noise.push(n / 2.0 + 1.0);
                atmo_noise.push((n + 1.0) / 2.0);
                n = (n + 1.0).max(0.5).min(0.8);
                // n = 0.8;
                println!("{}", n);
                verts[0] *= n as f32;
                verts[1] *= n as f32;
                verts[2] *= n as f32;
            }
        }
        bevy::render::mesh::VertexAttributeValues::Float4(_) => {}
    }
    // match mesh.attributes[1].values {
    //     bevy::render::mesh::VertexAttributeValues::Float(_) => {}
    //     bevy::render::mesh::VertexAttributeValues::Float2(_) => {}
    //     bevy::render::mesh::VertexAttributeValues::Float3(ref mut val) => {
    //         for (i, verts) in val.iter_mut().enumerate() {
    //             let n = noise[i];
    //             // println!("{}", n);
    //             verts[0] *= n as f32;
    //             verts[1] *= n as f32;
    //             verts[2] *= n as f32;
    //         }
    //     }
    //     bevy::render::mesh::VertexAttributeValues::Float4(_) => {}
    // }
    // COLOR RANDOMIZER
    match mesh.attributes[3].values {
        bevy::render::mesh::VertexAttributeValues::Float(_) => {}
        bevy::render::mesh::VertexAttributeValues::Float2(_) => {}
        bevy::render::mesh::VertexAttributeValues::Float4(ref mut val) => {
            for (i, verts) in val.iter_mut().enumerate() {
                let n = noise[i];
                let m = if i == 0 {
                    noise[noise.len() - 1]
                } else {
                    noise[i - 1]
                };
                let o = if i + 1 == noise.len() {
                    noise[0]
                } else {
                    noise[i + 1]
                };
                // println!("{}", n);
                // chose 1-n to make the planet colors darker because apparently my values are very close to 1
                verts[0] = 1.0 - m as f32;
                verts[1] = 1.0 - n as f32;
                verts[2] = 1.0 - o as f32;
            }
        }
        bevy::render::mesh::VertexAttributeValues::Float3(_) => {}
    }
    match atmo.attributes[3].values {
        bevy::render::mesh::VertexAttributeValues::Float(_) => {}
        bevy::render::mesh::VertexAttributeValues::Float2(_) => {}
        bevy::render::mesh::VertexAttributeValues::Float4(ref mut val) => {
            for (i, verts) in val.iter_mut().enumerate() {
                let n = if i == 0 {
                    atmo_noise[atmo_noise.len() - 1]
                } else {
                    atmo_noise[i - 1]
                };
                // println!("{}", n);
                verts[3] *= n as f32;
            }
        }
        bevy::render::mesh::VertexAttributeValues::Float3(_) => {}
    }

    let atmo_handle = meshes.add(atmo);
    let cube_handle = meshes.add(mesh);
    commands
        .spawn(MeshComponents {
            mesh: cube_handle,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                pipeline_handle,
                // NOTE: in the future you wont need to manually declare dynamic bindings
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 1,
                            binding: 0,
                        },
                        // StellarMaterial_basecolor
                        DynamicBinding {
                            bind_group: 1,
                            binding: 1,
                        },
                        // StellarMaterial_texture
                        DynamicBinding {
                            bind_group: 1,
                            binding: 2,
                        },
                        // StellarMaterial_atmo_radius
                        DynamicBinding {
                            bind_group: 1,
                            binding: 4,
                        },
                        // StellarMaterial_camera_pos
                        DynamicBinding {
                            bind_group: 1,
                            binding: 5,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(material)
        .spawn(MeshComponents {
            mesh: atmo_handle,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                pipeline_handle,
                // NOTE: in the future you wont need to manually declare dynamic bindings
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 1,
                            binding: 0,
                        },
                        // StellarMaterial_basecolor
                        DynamicBinding {
                            bind_group: 1,
                            binding: 1,
                        },
                        // StellarMaterial_texture
                        DynamicBinding {
                            bind_group: 1,
                            binding: 2,
                        },
                        // StellarMaterial_atmo_radius
                        DynamicBinding {
                            bind_group: 1,
                            binding: 4,
                        },
                        // StellarMaterial_camera_pos
                        DynamicBinding {
                            bind_group: 1,
                            binding: 5,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(material);

}

fn update_camera_pass_through(
    mut materials: ResMut<Assets<StellarMaterial>>,
    handle: Res<AssetHandles>,
    mut query: Query<(&Transform, &CameraMarker)>,
) {
    for (trans, _cam) in &mut query.iter() {
        if let Some(mat) = materials.get_mut(&handle.handle) {
            mat.camera_pos = trans.value;
        }
    }
}
