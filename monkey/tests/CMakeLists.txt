file(GLOB_RECURSE SRCS CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/*.cpp)

set(target monkey.t)

add_executable(${target} ${SRCS})

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip
)

FetchContent_MakeAvailable(googletest)

target_link_libraries(${target} PRIVATE
    monkey
    gtest_main
)

include(GoogleTest)
gtest_discover_tests(${target})
