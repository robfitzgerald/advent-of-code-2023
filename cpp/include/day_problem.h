#ifndef DAY_PROBLEM
#define DAY_PROBLEM

#include <string>

class DayProblem
{
private:
  int day;
  int problem;

public:
  DayProblem(int day, int problem) : day(day), problem(problem) {}
  int run(std::string filename);
};

#endif