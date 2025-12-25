#pragma once

#include "OpenGL.h"
#include "linmath.h"

class View {
private:
    mat4x4 view_projection_matrix;
    mat4x4 projection_matrix;
    mat4x4 view_matrix;

    mat4x4 model;

public:
    View(float fov);

    mat4x4* get_view_projection_matrix()
};