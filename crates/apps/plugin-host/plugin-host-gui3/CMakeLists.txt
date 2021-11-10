cmake_minimum_required(VERSION 3.20)
project(plugin_host_gui3)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(
    Qt6
    COMPONENTS
    Core
    Gui
    Widgets
    REQUIRED
)

add_executable(plugin_host_gui3 main.cpp)
target_link_libraries(
    plugin_host_gui3
    Qt::Core
    Qt::Gui
    Qt::Widgets
)
