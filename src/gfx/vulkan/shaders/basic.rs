pub(crate) mod vs {
    #![allow(dead_code)]

    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 color;
layout(location = 0) out vec3 fragColor;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
} uniforms;

void main() {
    gl_Position = uniforms.proj * uniforms.view * uniforms.world * vec4(position, 0.0, 1.0);
    fragColor = color;
}
"]
    struct Dummy;
}

pub(crate) mod fs {
    #![allow(dead_code)]

    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
#version 450

layout(location = 0) in vec3 fragColor;
layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(fragColor, 1.0);
}
"]
    struct Dummy;
}
