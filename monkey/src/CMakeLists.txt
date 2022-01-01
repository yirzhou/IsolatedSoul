set (target monkey)

add_library(${target} STATIC 
    token.cpp
    lexer.cpp
)

target_include_directories(${target} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")
