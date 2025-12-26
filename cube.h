#pragma once

#include "OpenGL.h"
#include <glm/mat4x4.hpp>

class Cube {
private:
    GLuint vertex_buffer;
    GLuint indice_buffer;

    glm::mat4 model;

public:
    Cube();

    void translate(glm::vec3 translate_value);
    void rotate(glm::vec3 rotate_value);

    void draw(glm::mat4& view_projection_matrix, GLuint mvp_location);
};