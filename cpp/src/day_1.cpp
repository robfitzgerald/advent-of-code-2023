#include <fstream>
#include <iostream>
#include <vector>
#include <tuple>
#include <numeric>
#include <unordered_map>
#include <optional>
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
    std::ifstream input_file(filename);
    if (!input_file.is_open())
    {
      std::cerr << "Error opening file" << filename << "." << std::endl;
      throw;
    }
    std::unordered_map<std::string, int> word_mapping;
    word_mapping["one"] = 1;
    word_mapping["two"] = 2;
    word_mapping["three"] = 3;
    word_mapping["four"] = 4;
    word_mapping["five"] = 5;
    word_mapping["six"] = 6;
    word_mapping["seven"] = 7;
    word_mapping["eight"] = 8;
    word_mapping["nine"] = 9;

    std::string line;
    std::vector<int> nums;

    while (std::getline(input_file, line))
    {
      std::tuple<int, int> result = day_1_helper::find_first_last_an(line, word_mapping);
      int num = day_1_helper::combine_first_last(result);
      nums.push_back(num);
    }

    int sum = std::accumulate(nums.begin(), nums.end(), 0);

    return sum;
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

  std::tuple<int, int> find_first_last_an(std::string line, std::unordered_map<std::string, int> mapping)
  {
    int first = -1, last;
    for (int i = 0; i < line.length(); i++)
    {
      std::optional<int> n = parse_int_from_pos(line, i, mapping);
      if (n.has_value())
      {
        int num = n.value();
        if (first == -1)
        {
          first = num;
        }
        last = num;
      }
    }

    return std::make_tuple(first, last);
  }

  std::optional<int> parse_int_from_pos(std::string line, int pos, std::unordered_map<std::string, int> mapping)
  {
    std::optional<int> result;
    size_t remaining = line.length() - pos;
    if (isdigit(line[pos]))
    {
      result = line[pos] - '0';
    }
    else if (mapping.count(line.substr(pos, 3)) > 0)
    {
      result = mapping[line.substr(pos, 3)];
    }
    else if (mapping.count(line.substr(pos, 4)) > 0)
    {
      result = mapping[line.substr(pos, 4)];
    }
    else if (mapping.count(line.substr(pos, 5)) > 0)
    {
      result = mapping[line.substr(pos, 5)];
    }

    return result;
  }
}
