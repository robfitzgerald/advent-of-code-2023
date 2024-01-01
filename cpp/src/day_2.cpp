#include <string>
#include <vector>
#include <fstream>
#include <iostream>
#include <sstream>
#include "day_2.h"

namespace day_2
{
  int problem_1(std::string filename)
  {
    Set *cover_target = new Set(14, 12, 13);

    std::vector<Game *> games = day_2_helper::get_games(filename);

    // std::cout << "games:" << std::endl;
    // for (Game *g : games)
    // {
    //   std::cout << *g;
    //   std::cout << std::endl;
    // }

    int valid_sum = 0;
    for (Game *game : games)
    {
      if (game->covered_by(cover_target))
      {
        valid_sum += game->get_id();
      }
    }

    return valid_sum;
  }
  int problem_2(std::string filename)
  {
    std::vector<Game *> games = day_2_helper::get_games(filename);
    std::vector<int> power_sets;
    for (Game *game : games)
    {
      Set *min_set = game->min_game();
      int power_set = min_set->power_set();
      power_sets.push_back(power_set);
    }

    int sum = 0;
    for (int num : power_sets)
    {
      sum += num;
    }
    return sum;
  }

  void Set::add(std::string color, int count)
  {
    if (color == "blue")
    {
      this->blue_ = count;
    }
    else if (color == "red")
    {
      this->red_ = count;
    }
    else if (color == "green")
    {
      this->green_ = count;
    }
    else
    {
      std::stringstream ss;
      ss << "cannot construct Set with color " << color << " and count " << count << std::endl;
      std::string msg = ss.str();
      throw new std::invalid_argument(msg);
    }
  }

  bool Set::covered_by(Set *that)
  {
    return this->blue_ <= that->blue_ &&
           this->red_ <= that->red_ &&
           this->green_ <= that->green_;
  }

  Set *Set::from_string(std::string row)
  {
    std::vector<std::string> cubes = day_2_helper::split(row, ',');
    // std::cout << "found " << cubes.size() << " comma-delimited entries." << std::endl;
    Set *set = new Set();
    for (std::vector<std::string>::iterator it = cubes.begin(); it != cubes.end(); ++it)
    {
      // std::cout << "  next entry: '" << *it << "'" << std::endl;
      std::string cube = day_2_helper::trim(*it);
      if (cube.length() == 0)
      {
        // std::cout << "  found empty cube!" << std::endl;
        continue;
      }
      std::vector<std::string> cube_info = day_2_helper::split(cube, ' ');
      std::string color = day_2_helper::trim(cube_info[1]);
      int count;
      try
      {
        count = std::stoi(day_2_helper::trim(cube_info[0]));
        // std::cout << "    found " << color << " with count " << count << std::endl;
      }
      catch (std::invalid_argument e)
      {
        std::cerr << "unable to parse count '" << cube_info[0] << "' for cube with color " << cube << std::endl;
        throw e;
      }

      set->add(color, count);
    }

    return set;
  }

  Game *Game::from_string(std::string row)
  {
    std::vector<std::string> game_split = day_2_helper::split(row, ':');
    int id = stoi(game_split[0].substr(5, game_split[0].length()));
    std::vector<Set *> sets;
    std::vector<std::string> sets_split = day_2_helper::split(game_split[1], ';');
    for (std::string set_str : sets_split)
    {
      Set *s = Set::from_string(set_str);
      sets.push_back(s);
    }
    Game *game = new Game(id, sets);
    return game;
  }

  bool Game::covered_by(Set *that)
  {
    bool result = true;
    for (Set *set : this->sets_)
    {
      result = result && set->covered_by(that);
    }
    return result;
  }

  Set *Game::min_game()
  {
    Set *min_set = new Set();
    for (Set *set : this->sets_)
    {
      min_set->agg_min(set);
    }
    return min_set;
  }

  void Set::agg_min(Set *that)
  {
    this->blue_ = (this->blue_ < that->blue_) ? that->blue_ : this->blue_;
    this->red_ = (this->red_ < that->red_) ? that->red_ : this->red_;
    this->green_ = (this->green_ < that->green_) ? that->green_ : this->green_;
  }
}

namespace day_2_helper
{

  std::vector<day_2::Game *> get_games(const std::string &filename)
  {
    using day_2::Game;
    std::ifstream input_file(filename);
    if (!input_file.is_open())
    {
      std::cerr << "Error opening file" << filename << "." << std::endl;
      throw;
    }
    std::string line;
    std::vector<Game *> games;

    while (std::getline(input_file, line))
    {
      Game *game = Game::from_string(line);
      games.push_back(game);
    }

    return games;
  }

  std::vector<std::string> split(const std::string &target, char c)
  {
    std::string temp;
    std::stringstream stringstream{target};
    std::vector<std::string> result;

    while (std::getline(stringstream, temp, c))
    {
      result.push_back(temp);
    }

    return result;
  }

  // std::vector<std::string> split(const std::string &target, char delimiter)
  // {
  //   std::vector<std::string> result;
  //   std::string::const_iterator start = target.begin();
  //   std::string::const_iterator end = target.end();

  //   while ((start = std::find(start, end, delimiter)) != end)
  //   {
  //     result.push_back(std::string(target.begin(), start));
  //     start++; // Skip the delimiter
  //   }

  //   result.push_back(std::string(start, end)); // Add the last token
  //   return result;
  // }

  std::string trim(const std::string &str)
  {
    if (str.length() == 0)
    {
      return str;
    }
    try
    {
      return str.substr(str.find_first_not_of(' '), str.find_last_not_of(' ') + 1);
    }
    catch (std::out_of_range e)
    {
      std::cerr << "failure during trim of '" << str << "' : " << std::endl;
      throw e;
    }
  }
}