#ifndef DAY_2
#define DAY_2

#include <string>
#include <vector>

namespace day_2
{
  enum AggregateMethod
  {
    Sum
  };

  class Set
  {
  public:
    Set(int blue, int red, int green) : blue_(blue), red_(red), green_(green) {}
    static Set from_string(std::string row);

  private:
    int blue_;
    int red_;
    int green_;
  };

  class Game
  {
  public:
    Game(int id, std::vector<Set> sets) : id_(id), sets_(sets) {}
    Set aggregate(AggregateMethod);
    static Game from_string(std::string);

  private:
    int id_;
    std::vector<Set> sets_;
  };

  int problem_1(std::string filename);
  int problem_2(std::string filename);
}

namespace day_2_helper
{

}

#endif