cmake_minimum_required(VERSION 3.14)


project(day16
    VERSION 1.0.0
    DESCRIPTION "Solution to day16"
    LANGUAGES CXX
)

add_executable(${PROJECT_NAME}
    day16.cc
)
target_sources(${PROJECT_NAME}
    PRIVATE
    day16.cc
)
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 17)
