#ifndef DAY_1
#define DAY_1

#include <string>
#include <tuple>

namespace day_1
{
  int problem_1(std::string filename);
  int problem_2(std::string filename);
}

namespace day_1_helper
{
  std::tuple<int, int> find_first_last_n(std::string line);
  int combine_first_last(std::tuple<int, int>);
}

#endif