#include "view.h"
#include <glm/ext/matrix_clip_space.hpp>
#include <glm/ext/matrix_transform.hpp>
#include <glm/gtc/quaternion.hpp>
#include <iostream>

const uint8_t PROJECTION_MUST_BE_RECALCULATED = 0b01;
const uint8_t VIEW_MUST_BE_RECALCULATED = 0b10;

View::View(ProjectionParameter projection_param, ViewParameter view_param)
{
    this->projection_param = projection_param;
    this->view_param = view_param;
    this->refresh_state = 0b11;
}

void View::update_fow_angle(float new_fov_angle)
{
    this->projection_param.fov_angle = new_fov_angle;
    this->refresh_state |= PROJECTION_MUST_BE_RECALCULATED;
}

void View::translate(glm::vec3 translate_value) {
    this->view_param.position += translate_value;
    this->refresh_state |= VIEW_MUST_BE_RECALCULATED;
}

void View::rotate(glm::vec3 rotate_value) {
    this->view_param.rotation += rotate_value;
    this->refresh_state |= VIEW_MUST_BE_RECALCULATED;
}

glm::mat4& View::get_view_projection_matrix()
{
    if (this->refresh_state != 0b00){
        if ((this->refresh_state & PROJECTION_MUST_BE_RECALCULATED) == PROJECTION_MUST_BE_RECALCULATED){
            //recalculate projection_matrix
            std::cout << "Recalculate projection_matrix" << std::endl;
            this->projection_matrix = glm::perspective(this->projection_param.fov_angle, this->projection_param.aspect, 0.1f, 100.f);

        }
        if ((this->refresh_state & VIEW_MUST_BE_RECALCULATED) == VIEW_MUST_BE_RECALCULATED) {
            //recalculate view_matrix
            std::cout << "Recalculate view_matrix" << std::endl;

            glm::quat quat_rotation = glm::quat(this->view_param.rotation);

            /*this->view_matrix =
                glm::translate(glm::mat4(1.0f), this->view_param.position) *
                glm::mat4(quat_rotation);*/

            this->view_matrix = glm::mat4(glm::conjugate(quat_rotation)) *
                glm::translate(glm::mat4(1.0f), -this->view_param.position);
        }

        std::cout << "Recalculate view_projection_matrix" << std::endl;
        this->view_projection_matrix = this->projection_matrix * this->view_matrix;
        this->refresh_state = 0b00;
    }

    return this->view_projection_matrix;
}
