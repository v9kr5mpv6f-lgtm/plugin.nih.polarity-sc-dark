cmake_minimum_required(VERSION 3.21)
project(slackwater_au VERSION 0.6.0 LANGUAGES C CXX OBJC OBJCXX)

# Wraps the Slackwater CLAP (built by `cargo xtask bundle slackwater --release` in the plugin repo)
# as an AUv2 effect. The CLAP is embedded in the bundle, so the .component is self-contained.
#   cmake -S au -B au/build -G Ninja -DSLACKWATER_CLAP=/path/to/Slackwater.clap
#   cmake --build au/build            # -> au/build/Slackwater.component (then codesign -s -)
set(SLACKWATER_CLAP "" CACHE PATH "Path to the built Slackwater.clap bundle")
if(NOT IS_DIRECTORY "${SLACKWATER_CLAP}")
  message(FATAL_ERROR "Set -DSLACKWATER_CLAP=<path to Slackwater.clap>")
endif()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_OBJCXX_STANDARD 20)
set(CMAKE_OSX_DEPLOYMENT_TARGET "10.13" CACHE STRING "")
set(CLAP_WRAPPER_DOWNLOAD_DEPENDENCIES ON CACHE BOOL "" FORCE)
set(CLAP_WRAPPER_BUILD_AUV2 ON CACHE BOOL "" FORCE)
include(FetchContent)
FetchContent_Declare(clap-wrapper
  GIT_REPOSITORY https://github.com/free-audio/clap-wrapper.git
  GIT_TAG v0.16.0)
FetchContent_MakeAvailable(clap-wrapper)

add_library(slackwater_as_auv2 MODULE)
target_add_auv2_wrapper(
  TARGET slackwater_as_auv2
  OUTPUT_NAME "Slackwater"
  BUNDLE_IDENTIFIER "com.oskillator.slackwater.auv2"
  BUNDLE_VERSION "${PROJECT_VERSION}"
  INSTRUMENT_TYPE "aufx"
  MANUFACTURER_NAME "Oskillator"
  MANUFACTURER_CODE "Oskl"
  SUBTYPE_CODE "Slkw"
  MACOS_EMBEDDED_CLAP_LOCATION "${SLACKWATER_CLAP}"
)
