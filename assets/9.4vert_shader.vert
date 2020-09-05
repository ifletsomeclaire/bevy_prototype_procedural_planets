#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv; //
layout(location = 1) out float v_height; // 
layout(location = 2) out vec4 v_color; //
layout(location = 3) out vec3 v_position; // center of the mesh

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    v_Uv = Vertex_Uv;
    vec3 center = vec3(Model[3][0], Model[3][1], Model[3][2]);
    v_height = distance(center, Vertex_Position);
    if(v_height > 35000) { 
        v_color = vec4(0.0, 0.5, 0.0, 1.0);
    } else {
        v_color = vec4(0.0, 0.0, 0.5, 1.0);
    }


    vec3 position = Vertex_Position;
    v_position = center + position;
    gl_Position = ViewProj * Model * vec4(position, 1.0);
}