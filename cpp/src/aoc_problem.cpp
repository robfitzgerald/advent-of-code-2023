#include "aoc_problem.h"
#include <iostream>
#include <stdexcept>
#include "day_1.h"

AocProblem get_aoc_problem(int day, int problem)
{
  int idx = ((day - 1) * 2) + problem;
  return AocProblem(idx);
}

int run(AocProblem problem, std::string filename)
{
  switch (problem)
  {
  case AocProblem::D1P1:
    return day_1::problem_1(filename);
  case AocProblem::D1P2:
    return day_1::problem_2(filename);
  default:
    std::cerr << "problem has no solution implemented: " << problem << "." << std::endl;
    throw std::invalid_argument("unknown day/problem");
  }
}

int get_day(AocProblem p)
{
  int idx = static_cast<int>(p);
  int day = ((idx - 1) / 2) + 1;
  return day;
}

int get_problem(AocProblem p)
{
  int idx = static_cast<int>(p);
  int problem = ((idx - 1) % 2) + 1;
  return problem;
}

inline std::ostream &operator<<(std::ostream &os, AocProblem &p)
{
  int day = get_day(p);
  int problem = get_problem(p);
  os << "day " << day << " problem " << problem;
  return os;
}
