cmake_minimum_required(VERSION 3.14)


option(WARNINGS_AS_ERROR "Enable Warings as Error" ON)


project(advent)

include(AnalyzerIncludes.cmake)



add_subdirectory(day1)
add_subdirectory(day4)


