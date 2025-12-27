#include "cube.h"
#include <glm/gtc/matrix_transform.hpp>
#include "vertex.h"
#include <iostream>

/*static const Vertex default_vertices[] =
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
};*/

static const Vertex default_vertices[] =
{
    //Face in front
    {{-0.5f,  0.5f, -0.5f}, {1.f, 1.f, 1.f}},//LEFT TOP - 0
    {{-0.5f, -0.5f, -0.5f}, {1.f, 1.f, 1.f}},//LEFT BOTTOM - 1
    {{ 0.5f,  0.5f, -0.5f}, {1.f, 1.f, 1.f}},//RIGHT TOP - 2
    {{ 0.5f, -0.5f, -0.5f}, {1.f, 1.f, 1.f}},//RIGHT BOTTOM - 3

    //Face behind the cube
    {{-0.5f,  0.5f, 0.5f}, {1.f, 1.f, 1.f}},//LEFT TOP - 4
    {{-0.5f, -0.5f, 0.5f}, {1.f, 1.f, 1.f} },//LEFT BOTTOM - 5
    {{ 0.5f,  0.5f, 0.5f}, {1.f, 1.f, 1.f}},//RIGHT TOP - 6 
    {{ 0.5f, -0.5f, 0.5f}, {1.f, 1.f, 1.f}},//RIGHT BOTTOM - 7
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

static bool default_cube_buffer_initialized = false;
static VBO_EBO default_cube_buffer;

Cube::Cube()
{
    this->model = glm::mat4(1.0);//identity matrix

    if (!default_cube_buffer_initialized) {
        glCreateBuffers(2, (uint32_t*)&default_cube_buffer);

        glNamedBufferStorage(default_cube_buffer.VBO, sizeof(default_vertices), default_vertices, NULL);
        glNamedBufferStorage(default_cube_buffer.EBO, sizeof(default_indices), default_indices, NULL);

        default_cube_buffer_initialized = true;
    }
}

void Cube::translate(glm::vec3 translate_value) {
    this->model = glm::translate(this->model, translate_value);
}

void Cube::rotate(glm::vec3 rotate_value)
{
    float max_angle = fmax(fmax(rotate_value.x, rotate_value.y), rotate_value.z);

    this->model = glm::rotate(this->model, max_angle, rotate_value / max_angle);
}

void Cube::draw(glm::mat4& view_projection_matrix, GLuint mvp_location, GLuint VAO)
{
    glm::mat4 mvp = view_projection_matrix * this->model;
    glUniformMatrix4fv(mvp_location, 1, GL_FALSE, (const GLfloat*)&mvp);

    glVertexArrayVertexBuffer(VAO, 0, default_cube_buffer.VBO, 0, 6*sizeof(float));
    glVertexArrayElementBuffer(VAO, default_cube_buffer.EBO);

    glDrawElements(GL_POINTS, sizeof(default_indices) / sizeof(default_indices[0]), GL_UNSIGNED_INT, 0);
}
