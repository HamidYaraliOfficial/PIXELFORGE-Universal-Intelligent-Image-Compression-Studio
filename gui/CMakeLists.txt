cmake_minimum_required(VERSION 3.25)
project(PIXELFORGE-GUI VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets Gui Core)

add_executable(pixelforge-gui
    src/main.cpp
    src/mainwindow.cpp
    src/backendclient.cpp
    include/mainwindow.h
    include/backendclient.h
)

target_include_directories(pixelforge-gui PRIVATE include)
target_link_libraries(pixelforge-gui PRIVATE Qt6::Widgets Qt6::Gui Qt6::Core)

if (WIN32)
    set_target_properties(pixelforge-gui PROPERTIES WIN32_EXECUTABLE TRUE)
endif()
