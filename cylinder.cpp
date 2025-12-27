#include "cylinder.h"
#include <glm/gtc/matrix_transform.hpp>
#include <glm/ext/scalar_constants.hpp>
#include "vertex.h"
#include <iostream>

Cylinder::Cylinder(uint32_t resolution)
{
    this->model = glm::mat4(1.0);//identity matrix

    this->resolution = resolution;

    size_t vertex_array_len = (2 + resolution * 2) * sizeof(Vertex);
    Vertex* vertex_array = (Vertex*)malloc(vertex_array_len);
    vertex_array[0] = {{0.0f, 0.5f, 0.0f}, {0.f, 1.f, 0.f}};//top center vertex
    vertex_array[1 + resolution] = { {0.0f, -0.5f, 0.0f}, {0.f, 0.f, 1.f} };//bottom center vertex

    size_t element_array_len = (2 + 2 + resolution * 2) * sizeof(Vertex);
    uint32_t* element_array = (uint32_t*)malloc(element_array_len);
    element_array[0] = 0;//top center vertex index
    element_array[1] = resolution;

    double angle_gap = 2 * glm::pi<double>() / resolution;
    double current_angle = 0.0;
    uint32_t element_i = 2;
    for (uint32_t vertex_i = 1; vertex_i < resolution + 1; vertex_i++) {
        float x = cos(current_angle);
        float y = sin(current_angle);
        std::cout << "x: " << x << " y: " << y << std::endl;
        vertex_array[vertex_i] = {//top vertex
            .pos = glm::vec3(x, 0.5, y),
            .col = glm::vec3(1.0, 0.0, 0.0)
        };
        vertex_array[vertex_i + 1 + resolution] = {//bottom vertex
            .pos = glm::vec3(x, -0.5, y),
            .col = glm::vec3(1.0, 0.0, 0.0)
        };
        element_array[element_i] = vertex_i;
        element_array[element_i + 1] = vertex_i + 1 + resolution;
        element_i += 2;
        current_angle += angle_gap;
    }
    element_array[element_i] = resolution + 2;
    element_array[element_i + 1] = resolution + 1;


    glCreateBuffers(2, (uint32_t*)&this->buffer);

    glNamedBufferStorage(this->buffer.VBO, vertex_array_len, vertex_array, NULL);
    glNamedBufferStorage(this->buffer.EBO, element_array_len, element_array, NULL);
}

void Cylinder::translate(glm::vec3 translate_value) {
    this->model = glm::translate(this->model, translate_value);
}

void Cylinder::rotate(glm::vec3 rotate_value)
{
    float max_angle = fmax(fmax(rotate_value.x, rotate_value.y), rotate_value.z);

    this->model = glm::rotate(this->model, max_angle, rotate_value / max_angle);
}

void Cylinder::draw(glm::mat4& view_projection_matrix, GLuint mvp_location, GLuint VAO, uint32_t nb_side_drawed)
{
    glm::mat4 mvp = view_projection_matrix * this->model;
    glUniformMatrix4fv(mvp_location, 1, GL_FALSE, (const GLfloat*)&mvp);

    glVertexArrayVertexBuffer(VAO, 0, this->buffer.VBO, 0, 6 * sizeof(float));
    glVertexArrayElementBuffer(VAO, this->buffer.EBO);

    glDrawArrays(GL_TRIANGLE_FAN, 0, 1 + this->resolution);//draw top face
    glDrawArrays(GL_TRIANGLE_FAN, 1 + this->resolution, 1 + this->resolution);//draw bottom face
    glDrawElements(GL_TRIANGLE_STRIP, /*2 + 2 + resolution * 2*/nb_side_drawed, GL_UNSIGNED_INT, 0);//draw all side
}
