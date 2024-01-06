#ifndef DAY_3
#define DAY_3

#include <string>

namespace day_3
{
  int problem_1(std::string filename);
  int problem_2(std::string filename);
  class Entry
  {
  public:
    Entry(char c) : _c(c), _g(std::nullopt) {}
    std::optional<int> get_number();
    void set_grouping(std::tuple<char, size_t, size_t>);
    std::optional<std::tuple<char, size_t, size_t>> get_grouping();

  private:
    char _c;
    std::optional<std::tuple<char, size_t, size_t>> _g;
  };

  class Schematic
  {
  public:
    Schematic(std::ifstream);
    void group_part_numbers();
    std::vector<int> get_part_numbers();

  private:
    std::vector<std::vector<Entry *>> entries;
  };
}

namespace day_3_helper
{

}

#endif