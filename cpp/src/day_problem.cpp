#include "day_problem.h"
#include <iostream>
#include "day_1.h"

int DayProblem::run(std::string filename)
{
  int result;
  if (this->problem < 1 || 2 < this->problem)
  {
    std::cerr << "problem value should be 1 or 2, found " << this->problem << "." << std::endl;
    throw;
  }

  if (this->day == 1 && this->problem == 1)
  {
    result = day_1::problem_1(filename);
  }
  else if (this->day == 1 && this->problem == 2)
  {
    result = day_1::problem_2(filename);
  }
  else
  {
    std::cerr << "unknown day " << this->day << " or problem " << this->problem << "." << std::endl;
    throw;
  }

  return result;
}