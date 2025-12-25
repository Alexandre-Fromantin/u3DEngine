#include "cube.h"

static const Vertex default_vertices[] =
{
    //Face in front
    {{-0.5f,  0.5f, -0.5f}, {0.f, 1.f, 1.f}},//LEFT TOP - 0
    {{-0.5f, -0.5f, -0.5f}, {0.f, 0.f, 1.f}},//LEFT BOTTOM - 1
    {{ 0.5f,  0.5f, -0.5f}, {1.f, 0.f, 0.f}},//RIGHT TOP - 2
    {{ 0.5f, -0.5f, -0.5f}, {0.f, 1.f, 0.f}},//RIGHT BOTTOM - 3

    //Face behind the cube
    {{-0.5f,  0.5f, 0.5f}, {0.f, 0.5f, 0.5f}},//LEFT TOP - 4
    {{-0.5f, -0.5f, 0.5f}, {0.f, 0.f, 0.5f}},//LEFT BOTTOM - 5
    {{ 0.5f,  0.5f, 0.5f}, {0.5f, 0.f, 0.f}},//RIGHT TOP - 6 
    {{ 0.5f, -0.5f, 0.5f}, {1.f, 0.5f, 1.f}},//RIGHT BOTTOM - 7
};

static const unsigned int default_indices[] = {
    //Face in front
    0, 1, 3,
    0, 3, 2,

    //Face behind the cube
    4, 5, 7,
    4, 7, 6,

    //Face on left
    4, 5, 1,
    4, 0, 1,

    //Face on right
    6, 7, 3,
    6, 2, 3,

    //Face on top
    0, 4, 6,
    0, 6, 2,

    //Face on bottom
    1, 5, 7,
    1, 7, 3,
};

Cube::Cube()
{
    mat4x4_identity(this->model);

    glGenBuffers(1, &this->vertex_buffer);
    glBindBuffer(GL_ARRAY_BUFFER, this->vertex_buffer);
    glBufferData(GL_ARRAY_BUFFER, sizeof(default_vertices), default_vertices, GL_STATIC_DRAW);

    glGenBuffers(1, &this->indice_buffer);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, this->indice_buffer);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(default_indices), default_indices, GL_STATIC_DRAW);
}

void Cube::translate(float x, float y, float z) {
    mat4x4_translate(this->model, x, y, z);
}

void Cube::rotate(float x, float y, float z)
{
    float max_angle = fmax(fmax(x, y), z);

    mat4x4_rotate(this->model, this->model, x / max_angle, y / max_angle, z / max_angle, max_angle);
}

void Cube::draw(mat4x4 view_projection_matrix, GLuint mvp_location)
{
    mat4x4 mvp;
    mat4x4_mul(mvp, view_projection_matrix, this->model);
    glUniformMatrix4fv(mvp_location, 1, GL_FALSE, (const GLfloat*)&mvp);

    glBindBuffer(GL_ARRAY_BUFFER, this->vertex_buffer);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, this->indice_buffer);

    glDrawElements(GL_TRIANGLES, sizeof(default_indices) / sizeof(default_indices[0]), GL_UNSIGNED_INT, 0);
}
