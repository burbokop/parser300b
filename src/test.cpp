
#include "lib.h"
#include <assert.h>
#include <iostream>
#include <gtest/gtest.h>

namespace memtest {
int parse_line(char* line){
    // This assumes that a digit will be found and the line ends in " Kb".
    int i = strlen(line);
    const char* p = line;
    while (*p <'0' || *p > '9') p++;
    line[i-3] = '\0';
    i = atoi(p);
    return i;
}

/**
 * @brief current_used_mem
 * @return memory in kb
 */
int current_used_mem(){
    FILE* file = fopen("/proc/self/status", "r");
    int result = -1;
    char line[128];

    while (fgets(line, 128, file) != NULL){
        if (strncmp(line, "VmSize:", 7) == 0){
            result = parse_line(line);
            break;
        }
    }
    fclose(file);
    return result;
}
}

struct TestToken {
    std::string name;

    const std::string& name_ref() const {
        return name;
    }
};

TEST(main_suite, memleak_test) {
    using namespace parser300b;

    const auto iteration = []{
        Grammar grammar = {
            .productions = {
                Production {
                    .lhs = "production1",
                    .rhs = {
                        Expression {
                            .terms = {
                                Term {
                                    .value = "term1",
                                    .is_terminal = false
                                },
                                Term {
                                    .value = "term2",
                                    .is_terminal = true
                                }
                            }
                        },
                        Expression {
                            .terms = {}
                        }
                    }
                },
                Production {
                    .lhs = "production2",
                    .rhs = {}
                }
            }
        };

        const std::vector<TestToken> tokens = {
            TestToken { .name = "token0" },
            TestToken { .name = "token1" }
        };

        parse(grammar, tokens);
    };

    for(int i = 0; i < 10; ++i) {
        const auto begin = memtest::current_used_mem();
        for(int j = 0; j < 10000; ++j) {
            iteration();
        }
        const auto leaked = memtest::current_used_mem() - begin;
        assert(!leaked);
    }
}


