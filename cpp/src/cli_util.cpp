#include <stdexcept>
#include <string>
#include <iostream>
#include "cli_util.h"

namespace cli_util
{
  int parse_int(std::string arg)
  {
    std::size_t pos;
    int x = std::stoi(arg, &pos);
    if (pos < arg.size())
    {
      std::cerr << "Trailing characters after number: " << arg << '\n';
    }
    return x;
    // try
    // {
    //   std::size_t pos;
    //   int x = std::stoi(arg, &pos);
    //   if (pos < arg.size())
    //   {
    //     std::cerr << "Trailing characters after number: " << arg << '\n';
    //   }
    //   return x;
    // }
    // catch (std::invalid_argument const &ex)
    // {
    //   std::cerr << "Invalid number: " << arg << '\n';
    //   throw
    // }
    // catch (std::out_of_range const &ex)
    // {
    //   std::cerr << "Number out of range: " << arg << '\n';
    // }
  }
}