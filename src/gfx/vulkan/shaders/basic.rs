pub(crate) mod vs {
    #![allow(dead_code)]

    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
#version 450

layout(location = 0) in vec3 vertex;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec3 fragColor;

layout(set = 0, binding = 0) uniform Global {
    mat4 view;
    mat4 projection;
    mat4 camera;
} global;

layout(set = 1, binding = 0) uniform Model {
    mat4 model;
} model;

void main() {
    gl_Position = global.projection * global.view * global.camera * model.model * vec4(vertex, 1.0);
    fragColor = vec3(1.0, 1.0, 1.0);
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
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(fragColor, 1.0);
}
"]
    struct Dummy;
}
