#ifndef DAY_1
#define DAY_1

#include <string>
#include <tuple>
#include <optional>
#include <unordered_map>

namespace day_1
{
  int problem_1(std::string filename);
  int problem_2(std::string filename);
}

namespace day_1_helper
{
  std::tuple<int, int> find_first_last_n(std::string line);
  std::tuple<int, int> find_first_last_an(std::string line, std::unordered_map<std::string, int>);
  int combine_first_last(std::tuple<int, int>);
  std::optional<int> parse_int_from_pos(std::string, int, std::unordered_map<std::string, int>);
}

#endif