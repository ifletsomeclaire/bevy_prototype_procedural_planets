use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::{vec2, vec3},
    prelude::*,
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

#[derive(RenderResources, ShaderDefs)]
struct StellarMaterial {
    pub basecolor: Color,
    #[shader_def]
    pub texture: Option<Handle<Texture>>,
    pub atmo_radius: f32,
    pub camera_pos: Mat4,
}
// #[derive(RenderResources, ShaderDefs)]
// struct QuadMaterial {
//     pub basecolor: Color,
//     #[shader_def]
//     pub texture: Option<Handle<Texture>>,
//     pub atmo_radius: f32,
//     pub camera_pos: Mat4,
// }

// #[derive(Default)]
// struct AssetHandles {
//     planet_handle: Handle<StellarMaterial>,
//     quad_handle: Handle<StellarMaterial>,
// }

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_asset::<StellarMaterial>()
        // .add_asset::<QuadMaterial>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_resource(CameraConfig::default())
        // .add_resource(AssetHandles::default())
        .add_plugin(wasd_camera::WasdCamera)
        .add_startup_system(setup.system())
        .add_system(update_camera_pass_through.system())
        .add_system_to_stage(
            stage::POST_UPDATE,
            asset_shader_defs_system::<StellarMaterial>.system(),
        )
        // .add_system_to_stage(
        //     stage::POST_UPDATE,
        //     asset_shader_defs_system::<QuadMaterial>.system(),
        // )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // asset_handles: Res<AssetHandles>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StellarMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let texture_handle = asset_server.load("assets/unscaledFinalPlanet.png").unwrap();

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
    let specialized_pipeline = RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
        pipeline_handle,
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
    )]);

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
    // let mut noise = Vec::new();
    let mut mesh = Mesh::from(shape::Icosphere {
        radius: 50000.,
        subdivisions: 20,
    });
    let mut distance = Vec::new();
    match mesh.attributes[0].values {
        bevy::render::mesh::VertexAttributeValues::Float3(ref mut val) => {
            for verts in val {
                let x = noise::RidgedMulti::new();
                let mut n = x.get([verts[0] as f64, verts[1] as f64, verts[2] as f64]);
                // noise.push(n / 2.0 + 1.0);
                n = (n + 1.0).max(0.5).min(0.8);
                // n = 0.8;
                println!("{}", n);
                verts[0] *= n as f32;
                verts[1] *= n as f32;
                verts[2] *= n as f32;
                distance.push((verts[0].powi(2) + verts[1].powi(2) + verts[2].powi(2)).sqrt());
            }
        }
        _ => {}
    }
    // COLOR RANDOMIZER
    match mesh.attributes[3].values {
        bevy::render::mesh::VertexAttributeValues::Float4(ref mut val) => {
            for (i, verts) in val.iter_mut().enumerate() {
                // let n = noise[i];
                // let m = if i == 0 {
                //     noise[noise.len() - 1]
                // } else {
                //     noise[i - 1]
                // };
                // let o = if i + 1 == noise.len() {
                //     noise[0]
                // } else {
                //     noise[i + 1]
                // };
                // chose 1-n to make the planet colors darker because apparently my values are very close to 1
                // verts[0] = 1.0 - m as f32;
                // verts[1] = 1.0 - n as f32;
                // verts[2] = 1.0 - o as f32;
                if distance[i] > 35000.0 {
                    verts[0] = 0.0;
                    verts[1] = 0.5;
                    verts[2] = 0.0;
                } else {
                    verts[0] = 0.0;
                    verts[1] = 0.0;
                    verts[2] = 0.5;
                }
            }
        }
        _ => {}
    }

    let cube_handle = meshes.add(mesh);
    commands
        .spawn(MeshComponents {
            mesh: cube_handle,
            render_pipelines: specialized_pipeline.clone(),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(material);
    let quad = Mesh::from(shape::Quad {
        size: vec2(10000.0, 5000.0),
        flip: false,
    });
    // let quad = Mesh::from(shape::Cube {
    //     size: 5000.0,
    // });

    let quad_mat = materials.add(StellarMaterial {
        basecolor: Color {
            r: 0.5,
            g: 0.0,
            b: 0.5,
            a: 0.1,
        },
        // texture: None,
        texture: Some(texture_handle),
        atmo_radius: 0.0,
        camera_pos: Transform::default().value,
    });
    let quad_handle = meshes.add(quad);
    commands
        .spawn(MeshComponents {
            mesh: quad_handle,
            render_pipelines: specialized_pipeline.clone(),
            translation: Translation::new(0.0, 0.0, 90000.0),
            ..Default::default()
        })
        .with(quad_mat);
}

fn update_camera_pass_through(
    mut materials: ResMut<Assets<StellarMaterial>>,
    // mut qmaterials: ResMut<Assets<QuadMaterial>>,
    mut query: Query<(&Transform, &CameraMarker)>,
) {
    for (trans, _cam) in &mut query.iter() {
        let mut handles = Vec::new();
        for mat in materials.iter() {
            handles.push(mat.0);
        }
        for handle in handles {
            if let Some(mat) = materials.get_mut(&handle) {
                mat.camera_pos = trans.value;
            }
        }
    }
}
