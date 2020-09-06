#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in float v_height;
layout(location = 2) in vec4 v_color;
layout(location = 3) in vec3 v_position;


layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 1) uniform StellarMaterial_basecolor {
    vec4 color;
};
layout(set = 1, binding = 2) uniform StellarMaterial_metallic {
    float metallic;
};
layout(set = 1, binding = 3) uniform StellarMaterial_roughness {
    float roughness;
};
layout(set = 1, binding = 4) uniform StellarMaterial_reflectance {
    float reflectance;
};
layout(set = 1, binding = 7) uniform StellarMaterial_atmo_radius {
    float atmo_radius;
};
layout(set = 1, binding = 8) uniform StellarMaterial_camera_pos {
    mat4 camera_mat;
};
# ifdef STELLARMATERIAL_TEXTURE 
layout(set = 1, binding = 5) uniform texture2D StellarMaterial_texture;
layout(set = 1, binding = 6) uniform sampler StellarMaterial_texture_sampler;
# endif


vec2 raySphere(vec3 sphereCenter, float sphereRadius, vec3 rayOrigin, vec3 rayDir) {
    vec3 offset = rayOrigin - sphereCenter;
    float a = dot(rayDir, rayDir);
    float b = 2 * dot(offset, rayDir);
    float c = dot(offset, offset) - sphereRadius * sphereRadius;
    float d = b * b - 4 * a * c;

    if (d > 0) {
        float s = sqrt(d);
        float dstToSphereNear = max(0, (-b - s) / (2 * a));
        float dstToSphereFar = (-b + s) / (2 * a);

        if (dstToSphereFar >= 0) {
            return vec2(dstToSphereNear, dstToSphereFar - dstToSphereNear);
        }
    }

    return vec2(2147483647., 0.0);
}


void main() {
    vec3 camera_pos = vec3(camera_mat[3][0], camera_mat[3][1], camera_mat[3][2]);
    vec3 rayDir = normalize(v_position - camera_pos);

    vec2 hitInfo = raySphere(v_position, atmo_radius, camera_pos, rayDir);
    float dstToAtmosphere = hitInfo.x;
    float dstThroughAtmosphere = hitInfo.y;

    vec4 acolor;
    if (dstToAtmosphere <= dstThroughAtmosphere) {
        acolor = vec4(0.1, 0.0, 0.0, dstThroughAtmosphere / (atmo_radius * 2.)) + v_color;
    } else {
        acolor = vec4(0.0, 0.1, 0.0, dstThroughAtmosphere / (atmo_radius * 2.)) + v_color;
    }

    // vec4 value = dstThroughAtmosphere / (atmo_radius * 2.) * vec4(rayDir * 0.5 + 0.5, 0.);
    // acolor = value;
# ifdef STELLARMATERIAL_TEXTURE
    color *= texture(
        sampler2D(StellarMaterial_texture, StellarMaterial_texture_sampler),
        v_Uv);
# endif
    // if(v_height > 40000) { 
    //     o_Target = color * v_height;
    // } else {
    //     o_Target = vec4(0.5, 0.0, 0.0, 1.0);
    // }
        o_Target = acolor;

}
