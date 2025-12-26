#pragma once

#include "OpenGL.h"
#include <glm/mat4x4.hpp>

struct ProjectionParameter {
    //In radians
    float fov_angle;

    //The ratio of the view dimension
    float aspect;
};

struct ViewParameter {
    //World position
    glm::vec3 position;

    //Values in radians
    glm::vec3 rotation;
};

class View {
private:
    glm::mat4 view_projection_matrix;

    ProjectionParameter projection_param;
    glm::mat4 projection_matrix;

    ViewParameter view_param;
    glm::mat4 view_matrix;

    //first bit on 1 -> projection_matrix must be recalculated
    //second bit on 1 -> view_matrix must be recalculated
    uint8_t refresh_state;

public:
    View(ProjectionParameter projection_param, ViewParameter view_param);

    void update_fow_angle(float new_fov_angle);
    void translate(glm::vec3 translate_value);
    void rotate(glm::vec3 rotate_value);

    glm::mat4& get_view_projection_matrix();
};