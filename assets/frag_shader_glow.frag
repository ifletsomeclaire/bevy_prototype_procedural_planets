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

void main() {
    vec4 acolor = color;
# ifdef STELLARMATERIAL_TEXTURE
    acolor *= texture(
        sampler2D(StellarMaterial_texture, StellarMaterial_texture_sampler),
        v_Uv);
# endif

	vec2 st = gl_FragCoord.xy/vec2(1100, 600);
// Normalized pixel coordinates (from 0 to 1)
    // The ratio of the width and height of the screen
    float widthHeightRatio = 1100/600;
    vec2 centre = vec2(0.5, 0.5);
	// Position of fragment relative to centre of screen
    vec2 pos = centre - st;
    // Adjust y by ratio for uniform transforms
    pos.y /= widthHeightRatio;
    
    //**********         Glow        **********
    
    // Equation 1/x gives a hyperbola which is a nice shape to use for drawing glow as 
    // it is intense near 0 followed by a rapid fall off and an eventual slow fade
    float dist = 1./length(pos);
    
    //**********        Radius       **********
    
    // Dampen the glow to control the radius
    dist *= 0.1;
    
    //**********       Intensity     **********
    
    // Raising the result to a power allows us to change the glow fade behaviour
    // See https://www.desmos.com/calculator/eecd6kmwy9 for an illustration
    // (Move the slider of m to see different fade rates)
    dist = pow(dist, 0.8);
    
    //Knowing the distance from a fragment to the source of the glow, the above can be written compactly as: 
    //	float getGlow(float dist, float radius, float intensity){
    //		return pow(radius/dist, intensity);
	//	}
    //The returned value can then be multiplied with a colour to get the final result
       
    // Get colour
    vec3 col = dist * vec3(1.0, 0.5, 0.25);
	
    // See comment by P_Malin
    col = 1.0 - exp( -col );
    
    // Output to screen
    acolor = vec4(col, 1.0);

    o_Target = v_color * acolor;
}
