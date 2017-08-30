pub(crate) mod vs {
    #![allow(dead_code)]

    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
#version 450

layout(location = 0) in vec3 vertex;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec3 v_normal;

layout(set = 0, binding = 0) uniform Global {
    mat4 camera;
    mat4 view;
    mat4 projection;
} global;

layout(set = 1, binding = 0) uniform Model {
    mat4 model;
} model;

void main() {
    mat4 worldview = global.view * global.camera;
    gl_Position = global.projection * worldview * model.model * vec4(vertex, 1.0);
    v_normal = transpose(inverse(mat3(worldview))) * normal;
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

layout(location = 0) in vec3 v_normal;
layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, 0.0, 1.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    vec3 dark_color = vec3(0.6, 0.0, 0.0);
    vec3 regular_color = vec3(1.0, 0.0, 0.0);
    f_color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
"]
    struct Dummy;
}
