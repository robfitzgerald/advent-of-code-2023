#include <fstream>
#include <iostream>
#include <vector>
#include <tuple>
#include <numeric>
#include "day_1.h"

namespace day_1
{
  int problem_1(std::string filename)
  {
    std::ifstream input_file(filename);
    if (!input_file.is_open())
    {
      std::cerr << "Error opening file" << filename << "." << std::endl;
      throw;
    }
    std::string line;
    std::vector<int> nums;

    while (std::getline(input_file, line))
    {
      std::tuple<int, int> result = day_1_helper::find_first_last_n(line);
      int num = day_1_helper::combine_first_last(result);
      nums.push_back(num);
    }

    int sum = std::accumulate(nums.begin(), nums.end(), 0);

    return sum;
  }

  int problem_2(std::string filename)
  {
    return 2;
  }
}

namespace day_1_helper
{
  std::tuple<int, int> find_first_last_n(std::string line)
  {
    int first = -1, last;
    for (char c : line)
    {
      if (isdigit(c))
      {
        int num = c - '0';
        if (first == -1)
        {
          first = num;
        }
        last = num;
      }
    }
    return std::make_tuple(first, last);
  }

  int combine_first_last(std::tuple<int, int> pair)
  {
    auto [first, last] = pair;
    return first * 10 + last;
  }
}