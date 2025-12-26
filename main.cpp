#include "main.h"
#include "view.h"
#include <glm/ext/matrix_transform.hpp>
#include <glm/ext/matrix_clip_space.hpp>

static const char* vertex_shader_text =
"#version 330\n"
"uniform mat4 mvp;\n"
"in vec3 vCol;\n"
"in vec3 vPos;\n"
"out vec3 color;\n"
"void main()\n"
"{\n"
"    gl_Position = mvp * vec4(vPos, 1.0);\n"
"    color = vCol;\n"
"}\n";

static const char* fragment_shader_text =
"#version 330\n"
"in vec3 color;\n"
"out vec4 fragment;\n"
"void main()\n"
"{\n"
"    fragment = vec4(color, 1.0);\n"
"}\n";

View* main_view_ptr = nullptr;

static void error_callback(int error, const char* description)
{
    fprintf(stderr, "Error: %s\n", description);
}

static void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods)
{
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
        glfwSetWindowShouldClose(window, GLFW_TRUE);

    if (main_view_ptr && (action == GLFW_PRESS || action == GLFW_REPEAT)){
        if (key == GLFW_KEY_UP && action == GLFW_REPEAT) {
            main_view_ptr->translate(glm::vec3(0.0, 0.0, -0.1));
        }
        else if (key == GLFW_KEY_DOWN && action == GLFW_REPEAT) {
            main_view_ptr->translate(glm::vec3(0.0, 0.0, 0.1));
        }
        else if (key == GLFW_KEY_LEFT && action == GLFW_REPEAT) {
            main_view_ptr->translate(glm::vec3(-0.1, 0.0, 0.0));
        }
        else if (key == GLFW_KEY_RIGHT && action == GLFW_REPEAT) {
            main_view_ptr->translate(glm::vec3(0.1, 0.0, 0.0));
        }
    }
}

#define USE_SECOND_FULL_SCREEN

int main(void)
{
    glfwSetErrorCallback(error_callback);

    if (!glfwInit())
        exit(EXIT_FAILURE);

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

#ifdef USE_SECOND_FULL_SCREEN
    int monitor_count = 0;
    GLFWmonitor** all_monitor_ptr = glfwGetMonitors(&monitor_count);
    GLFWmonitor* second_monitor_ptr = all_monitor_ptr[1];
    const GLFWvidmode* monitor_video_mode = glfwGetVideoMode(second_monitor_ptr);

    glfwWindowHint(GLFW_CENTER_CURSOR, GLFW_FALSE);
    glfwWindowHint(GLFW_AUTO_ICONIFY, GLFW_FALSE);
    GLFWwindow* window = glfwCreateWindow(monitor_video_mode->width, monitor_video_mode->height, "OpenGL Window", second_monitor_ptr, NULL);
#else
    GLFWwindow* window = glfwCreateWindow(700, 700, "OpenGL Window", NULL, NULL);
#endif
    if (!window)
    {
        glfwTerminate();
        exit(EXIT_FAILURE);
    }

    glfwSetKeyCallback(window, key_callback);

    glfwMakeContextCurrent(window);
    gladLoadGL(glfwGetProcAddress);
    glfwSwapInterval(1);//vsync

    int width, height;
    glfwGetFramebufferSize(window, &width, &height);
    const float aspect = width / (float)height;

    glViewport(0, 0, width, height);

    glEnable(GL_DEPTH_TEST);

    ProjectionParameter projection_param = {
            .fov_angle = glm::radians(90.f),
            .aspect = aspect
    };
    ViewParameter view_param = {
            .position = glm::vec3(0.0, 0.0, 0.0),
            .rotation = glm::vec3(0.0, 0.0, 0.0)
    };

    Cube cube = Cube();
    View view = View(projection_param, view_param);

    main_view_ptr = &view;

    const GLuint vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertex_shader, 1, &vertex_shader_text, NULL);
    glCompileShader(vertex_shader);

    const GLuint fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragment_shader, 1, &fragment_shader_text, NULL);
    glCompileShader(fragment_shader);

    const GLuint program = glCreateProgram();
    glAttachShader(program, vertex_shader);
    glAttachShader(program, fragment_shader);
    glLinkProgram(program);

    const GLint mvp_location = glGetUniformLocation(program, "mvp");
    const GLint vpos_location = glGetAttribLocation(program, "vPos");
    const GLint vcol_location = glGetAttribLocation(program, "vCol");

    GLuint vertex_array;
    glGenVertexArrays(1, &vertex_array);
    glBindVertexArray(vertex_array);

    glEnableVertexAttribArray(vpos_location);
    glVertexAttribPointer(vpos_location, 3, GL_FLOAT, GL_FALSE,
        sizeof(Vertex), (void*)offsetof(Vertex, pos));

    glEnableVertexAttribArray(vcol_location);
    glVertexAttribPointer(vcol_location, 3, GL_FLOAT, GL_FALSE,
        sizeof(Vertex), (void*)offsetof(Vertex, col));

    cube.translate(glm::vec3(0, 0, -2));

    while (!glfwWindowShouldClose(window))
    {
        //cube.rotate(glm::vec3(0.f, 0.05f, 0.1f));

        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        glUseProgram(program);
        glBindVertexArray(vertex_array);

        glm::mat4 view_projection_matrix = view.get_view_projection_matrix();

        cube.draw(view_projection_matrix, mvp_location);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwDestroyWindow(window);

    glfwTerminate();
    exit(EXIT_SUCCESS);
}
