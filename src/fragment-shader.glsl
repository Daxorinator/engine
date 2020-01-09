// Vertex Shader Output becomes Fragment Shader input which has been rasterized
// and interpolated (Input)
in vec3 vertex_color;

// Output will be a single color (Output)
out vec3 fragment_color;

//OpenGL Shader Language Application Entrypoint
void main() {
    //Fragment Color is same as Vertex Color
    fragment_color = vertex_color;
}