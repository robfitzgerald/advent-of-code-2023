#ifndef AOC_PROBLEM
#define AOC_PROBLEM

#include <string>

enum class AocProblem : int
{
  D1P1 = 1,
  D1P2,
  D2P1,
  D2P2,
  D3P1,
  D3P2,
  D4P1,
  D4P2
};

AocProblem get_aoc_problem(int day, int problem);
int run(AocProblem problem, std::string filename);
inline std::ostream &operator<<(std::ostream &os, AocProblem &p);
int get_day(AocProblem p);
int get_problem(AocProblem p);

#endif