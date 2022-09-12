# CMakeLists files in this project can
# refer to the root source directory of the project as ${HELLO_SOURCE_DIR} and
# to the root binary directory of the project as ${HELLO_BINARY_DIR}.
cmake_minimum_required (VERSION 2.8.11)
project (vk-cholesky)

find_package(Vulkan REQUIRED)
# add_library(vk-cholesky ./src/vkcontext.c)
add_executable(vk-cholesky ./src/vkcontext.cpp)
target_include_directories(vk-cholesky PUBLIC Vulkan::Headers)
target_link_libraries(vk-cholesky Vulkan::Vulkan)