#pragma once

#include "OpenGL.h"
#include "vertex.h"

class Cube {
private:
    GLuint vertex_buffer;
    GLuint indice_buffer;

    mat4x4 model;

public:
    Cube();

    void translate(float x, float y, float z);
    void rotate(float x, float y, float z);

    void draw(mat4x4 view_projection, GLuint mvp_location);
};