option(USE_CLANG_TIDY "Use ClangTidy for static code analysis" ON)
option(USE_VALGRIND "Use Valgrind in MemCheck tests" OFF)
option(USE_ADDRESSSANITIZER "Detects addressability issues" OFF)
option(USE_LEAKSANITIZER "LeakSanitizer (detects memory leaks)" OFF)
option(USE_THREADSANITIZER "Detects data races and deadlocks" OFF)
option(USE_MEMORYSANITIZER "Detects use of uninitialized memory" OFF)
option(USE_UNDEFINEDBEHAVIORSANITIZER "UBSan modifies the program at compile-time to catch various kinds of undefined behavior during program execution" OFF)

message(STATUS "USE_CLANG_TIDY? ${USE_CLANG_TIDY}")
message(STATUS "USE_VALGRIND? ${USE_VALGRIND}")
message(STATUS "USE_THREADSANITIZER? ${USE_THREADSANITIZER}")
message(STATUS "USE_MEMORYSANITIZER? ${USE_MEMORYSANITIZER}")
message(STATUS "USE_LEAKSANITIZER? ${USE_LEAKSANITIZER}")
message(STATUS "USE_ADDRESSSANITIZER? ${USE_ADDRESSSANITIZER}")

if(USE_CLANG_TIDY)
    # Usage
    #if(USE_CLANG_TIDY)
    #   set_target_properties(${name}
    #       PROPERTIES
    #           CXX_CLANG_TIDY "${DO_CLANG_TIDY}"
    #)
    #endif()

    find_program(CLANG_TIDY_BIN
        NAMES "clang-tidy"
        PATHS "/usr/bin"
    )


    if(NOT CLANG_TIDY_BIN)
        message(FATAL_ERROR "clang-tidy not found.")
    else()
        set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
        message(STATUS "clang-tidy found: ${CLANG_TIDY_BIN}")
        set(DO_CLANG_TIDY "${CLANG_TIDY_BIN}" "-checks=*,-clang-analyzer-alpha.*")
        message(STATUS "clang-tidy args: ${DO_CLANG_TIDY}")
    endif()
endif()

# LLVM/Google Santizers

# Need -fno-omit-frame-pointer to allow the backtraces to be symbolified.
set(SANITIZER_FLAGS "-g -O1 -fno-omit-frame-pointer")

if (${USE_VALGRIND})
    find_program(MEMORYCHECK_COMMAND NAMES valgrind)
    set( MEMORYCHECK_COMMAND_OPTIONS "--trace-children=yes --leak-check=full" )
    set( MEMORYCHECK_SUPPRESSIONS_FILE "${PROJECT_SOURCE_DIR}/valgrind_suppress.txt" )
    message(STATUS "MemCheck will use ${MEMORYCHECK_COMMAND} ${MEMORYCHECK_COMMAND_OPTIONS} ${MEMORYCHECK_SUPPRESSIONS_FILE}")
    message(STATUS "Read the Documentation for the meaning of these flags: http://valgrind.org/docs/manual/mc-manual.html")
    set(USE_MEM_CHECK ON)
endif()

if(${USE_ADDRESSSANITIZER})
    set(CTEST_MEMORYCHECK_TYPE "AddressSanitizer")
    set(SANITIZER_FLAGS "${SANITIZER_FLAGS} -fsanitize=address")
    set(USE_MEM_CHECK ON)
endif()

if(${USE_LEAKSANITIZER})
    set(CTEST_MEMORYCHECK_TYPE "LeakSanitizer")
    set(SANITIZER_FLAGS "${SANITIZER_FLAGS} -fsanitize=address")
    set(USE_MEM_CHECK ON)
endif()

if(${USE_THREADSANITIZER})
    set(CTEST_MEMORYCHECK_TYPE "ThreadSanitizer")
    set(SANITIZER_FLAGS "${SANITIZER_FLAGS} -fsanitize=thread")
    set(USE_MEM_CHECK ON)
endif()

if(${USE_MEMORYSANITIZER})
    set(CTEST_MEMORYCHECK_TYPE "MemorySanitizer")
    set(SANITIZER_FLAGS "${SANITIZER_FLAGS} -fsanitize=memory")
    set(USE_MEM_CHECK ON)
endif()

if(${USE_MEM_CHECK})
    message(STATUS "Using ${CTEST_MEMORYCHECK_TYPE} with ${MEMORYCHECK_COMMAND} and args ${SANITIZER_FLAGS}")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${SANITIZER_FLAGS}")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${SANITIZER_FLAGS}")
    set(CMAKE_CGO_LDFLAGS "${CMAKE_CGO_LDFLAGS} ${SANITIZER_FLAGS}")
endif()
