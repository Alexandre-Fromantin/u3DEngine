# CMakeList.txt : projet CMake pour OpenGL, incluez la source et définissez
# la logique spécifique au projet ici.
#
cmake_minimum_required (VERSION 3.20)

# Activez Rechargement à chaud pour les compilateurs MSVC si cela est pris en charge.
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

project ("u3DEngine")

add_subdirectory(${CMAKE_SOURCE_DIR}/glad/cmake)
glad_add_library(glad_gl_core_33 SHARED LOCATION ${CMAKE_SOURCE_DIR}/glad_generated API gl:core=3.3)

add_subdirectory(${CMAKE_SOURCE_DIR}/glfw)
find_package(OpenGL REQUIRED)

add_executable (${PROJECT_NAME} "main.cpp"  "linmath.h" "cube.cpp" "cube.h" "vertex.h" "main.h" "OpenGL.h" "view.h" "view.cpp")

set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 23)
 
target_link_libraries(${PROJECT_NAME} PRIVATE glad_gl_core_33 glfw OpenGL::GL)

target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/glad_generated/include)