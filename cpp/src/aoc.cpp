#include <iostream>
#include <vector>
#include <string>
#include <stdexcept>
#include "cli_util.h"
#include <format>
#include "day_1.h"
// #include "day_problem.h"
#include "aoc_problem.h"

int main(int argc, char *argv[])
{
    if (argc != 4)
    {
        std::cerr << "usage: <problem #> <part #> <filename>";
        return EXIT_FAILURE;
    }

    // these should be numeric
    int day = cli_util::parse_int(argv[1]);
    int problem = cli_util::parse_int(argv[2]);
    std::string filename = argv[3];
    // DayProblem *dp = new DayProblem(day, problem);
    AocProblem p = get_aoc_problem(day, problem);

    std::cout << "running day " << day << " problem " << problem;
    std::cout << " with file " << filename << "." << std::endl;

    int result = run(p, filename);

    std::cout << "result is '" << result << "'." << std::endl;
    return EXIT_SUCCESS;
}