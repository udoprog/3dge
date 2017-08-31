pub(crate) mod vs {
    #![allow(dead_code)]

    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coord;

layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec2 v_tex_coord;

layout(set = 0, binding = 0) uniform Global {
    mat4 camera;
    mat4 view;
    mat4 projection;
} global;

layout(set = 0, binding = 1) uniform Model {
    mat4 model;
} model;

void main() {
    mat4 worldview = global.view * global.camera;
    gl_Position = global.projection * worldview * model.model * vec4(position, 1.0);
    v_normal = transpose(inverse(mat3(worldview))) * normal;
    v_tex_coord = tex_coord;
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

layout(location = 0) in vec3 normal;
layout(location = 1) in vec2 tex_coord;
layout(location = 0) out vec4 f_color;

layout(set = 1, binding = 0) uniform sampler2D tex;

const vec3 LIGHT = vec3(0.0, 1.0, 1.0);

void main() {
    float brightness = dot(normalize(normal), normalize(LIGHT));
    vec4 dark_color = vec4(0.6, 0.0, 0.0, 1.0);
    vec4 color = texture(tex, tex_coord);
    f_color = color;
}
"]
    struct Dummy;
}
