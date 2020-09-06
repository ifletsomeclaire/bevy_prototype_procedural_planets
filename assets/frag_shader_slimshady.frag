#version 450

in vec4 gl_FragCoord;

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in float v_height;
layout(location = 2) in vec4 v_color;
layout(location = 3) in vec3 v_position;
layout(location = 4) in vec3 v_center;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 1) uniform StellarMaterial_basecolor {
    vec4 color;
};
layout(set = 1, binding = 4) uniform StellarMaterial_atmo_radius {
    float atmo_radius;
};
layout(set = 1, binding = 5) uniform StellarMaterial_camera_pos {
    mat4 camera_mat;
};
# ifdef STELLARMATERIAL_TEXTURE 
layout(set = 1, binding = 2) uniform texture2D StellarMaterial_texture;
layout(set = 1, binding = 3) uniform sampler StellarMaterial_texture_sampler;
# endif

// Plot a line on Y using a value between 0.0-1.0
float plot(vec2 st) {    
    return smoothstep(0.30, 0.0, abs(st.y - st.x));
}
float plottwo(vec2 st) {    
    return smoothstep(0.30, 0.0, abs(st.x - st.y));
}




void main() {
    vec4 acolor = color;
# ifdef STELLARMATERIAL_TEXTURE
    acolor *= texture(
        sampler2D(StellarMaterial_texture, StellarMaterial_texture_sampler),
        v_Uv);
# endif

	vec2 st = gl_FragCoord.xy/vec2(600, 600);
    float y = st.x;

    vec3 color = vec3(y);
    vec3 colortwo = vec3(y);

    // Plot a line
    float pct = plot(st);
    color = (0.2-pct)*color+pct*vec3(2.0,1.0,0.0);
    float pcttwo = plottwo(st);
    colortwo = (0.2-pcttwo)*colortwo+pcttwo*vec3(2.0,1.0,0.0);

	acolor = vec4(color * colortwo,0.5);


    o_Target = v_color * acolor;
}
