#pragma once

#include "OpenGL.h"
#include <glm/mat4x4.hpp>

class Cylinder {
private:
    VBO_EBO buffer;

    glm::mat4 model;

    uint32_t resolution;

public:
    Cylinder(uint32_t resolution);

    void translate(glm::vec3 translate_value);
    void rotate(glm::vec3 rotate_value);

    void draw(glm::mat4& view_projection_matrix, GLuint mvp_location, GLuint VAO, uint32_t nb_side_drawed);
};