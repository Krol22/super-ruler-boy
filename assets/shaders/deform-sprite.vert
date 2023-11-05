#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float width;
    float height;
};

layout(set = 2, binding = 0) uniform Mesh {
    mat4 Model;
    mat4 InverseTransposeModel;
    uint flags;
};

layout(set = 1, binding = 0) uniform CustomMaterial {
    vec4 Color;
    vec2 deformation;
};

const float sideWaysDeformationFactor = 5;

void main() {
    vec2 newVertex_Uv = Vertex_Uv;
    vec3 newVertex_Position = Vertex_Position;

    vec2 deformationStrength = abs(deformation);
    float sideWaysDeformation = min(deformationStrength.x, deformationStrength.y);
    float spriteWidth = abs(newVertex_Position.x);

    if (sign(newVertex_Position.y) != sign(deformation.y)) {
        newVertex_Position.x += sideWaysDeformation * sideWaysDeformationFactor * spriteWidth * sign(deformation.x);
    }

    vec2 scale = vec2(1.0) - deformationStrength;
    newVertex_Position.x *= scale.x / scale.y;
    newVertex_Position.y *= scale.y / scale.x;

    v_Uv = newVertex_Uv;
    gl_Position = ViewProj * Model * vec4(newVertex_Position, 1.0);
}
