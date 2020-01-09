// Specify vertex attributes (Inputs)
in vec2 position;
in vec3 color;

// Specify output of shader (Outputs)
out vec3 vertex_color;

//OpenGL Shader Language Application Entrypoint
void main() {
    //Forward our vertex colors
    vertex_color = color;

    //Use the position vertex attribute to create a gl_Position in space
    //This position is expressed through a Vector4 value, with the format:
    // (x, y, z, w) - The Homogenous Coordinate System
    gl_Position = vec4(position, 0.0, 1.0);
}