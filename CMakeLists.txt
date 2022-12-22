# CMakeLists files in this project can
# refer to the root source directory of the project as ${HELLO_SOURCE_DIR} and
# to the root binary directory of the project as ${HELLO_BINARY_DIR}.
cmake_minimum_required (VERSION 2.8.11)
project(vk-cholesky)

find_package(Vulkan REQUIRED)

set(
   SOURCE
   ./src/vkcholesky.hpp
   ./src/vkcholesky.cpp
)

add_library(vkcholesky SHARED ${SOURCE})
target_include_directories(vkcholesky PUBLIC Vulkan::Headers)
target_link_libraries(vkcholesky PUBLIC Vulkan::Vulkan)

install(TARGETS vkcholesky DESTINATION ${CMAKE_CURRENT_SOURCE_DIR})