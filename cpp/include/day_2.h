#ifndef DAY_2
#define DAY_2

#include <string>
#include <vector>
#include <iostream>

namespace day_2
{
  enum AggregateMethod
  {
    Sum
  };

  class Set
  {
  public:
    Set() : blue_(0), red_(0), green_(0) {}
    Set(int blue, int red, int green) : blue_(blue), red_(red), green_(green) {}
    void add(std::string, int);
    void agg_min(Set *);
    static Set *from_string(std::string row);
    bool covered_by(Set *);
    inline friend std::ostream &operator<<(std::ostream &os, const Set &set)
    {
      os << "(B=" << set.blue_ << ", R=" << set.red_ << ", G=" << set.green_ << ")";
      return os;
    }
    inline int power_set()
    {
      return this->blue_ * this->green_ * this->red_;
    }

  private:
    int blue_;
    int red_;
    int green_;
  };

  class Game
  {
  public:
    int get_id() { return this->id_; }
    Game(int id, std::vector<Set *> sets) : id_(id), sets_(sets) {}
    static Game *from_string(std::string);
    bool covered_by(Set *);
    Set *min_game();
    inline friend std::ostream &operator<<(std::ostream &os, const Game &game)
    {
      os << "Game " << game.id_ << "<";
      for (Set *set : game.sets_)
      {
        os << *set << " ";
      }
      os << ">";
      return os;
    }

  private:
    int id_;
    std::vector<Set *> sets_;
  };

  int problem_1(std::string filename);
  int problem_2(std::string filename);
}

namespace day_2_helper
{
  std::vector<day_2::Game *> get_games(const std::string &);
  std::vector<std::string> split(const std::string &, char);
  std::string trim(const std::string &);
}

#endif