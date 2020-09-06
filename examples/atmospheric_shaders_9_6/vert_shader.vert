#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 3) in vec4 Vertex_Color;

layout(location = 0) out vec2 v_Uv; // uv value.....
layout(location = 1) out float v_height; // distance from position to center
layout(location = 2) out vec4 v_color; // color of the vertex
layout(location = 3) out vec3 v_position; // position of the vertex
layout(location = 4) out vec3 v_center; // center of the mesh

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 1, binding = 5) uniform StellarMaterial_camera_pos {
    mat4 camera_mat;
};

void main() {
    vec3 cam_pos = vec3(camera_mat[3]);
    float dist = distance(cam_pos, v_position);

    v_Uv = Vertex_Uv;
    vec3 center = vec3(Model[3]);
    // vec3 center = vec3(Model[3][0], Model[3][1], Model[3][2]);
    v_height = distance(center, Vertex_Position);
    v_color = Vertex_Color;
    v_center = center;
    v_position = Vertex_Position + center;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}