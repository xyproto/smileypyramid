#include <cstddef>
#include <iostream>
#include <sstream>
#include <string>

#include "docopt.h"

using namespace std;
using namespace std::string_literals;

using width_t = uint64_t;

static const auto DEFAULT_WIDTH = width_t{ 10 };

static const auto versionString = "Smiley Pyramid 1.0"s;
static const auto USAGE = versionString + R"(

    Usage:
      smileypyramid <width>
      smileypyramid (-h | --help)
      smileypyramid --version

    Options:
      -h --help     Show this screen.
      --version     Show version.
)";

auto smiley_line(const width_t length) -> const std::string
{
    if (length == 1) {
        return ")"s;
    }
    auto ss = std::stringstream{};
    auto l = length;
    while (l > 1) {
        if (l % 9 == 0) {
            ss << ":-):-):-)"s;
            l -= 9;
        } else if (l % 5 == 0) {
            ss << ":-):)"s;
            l -= 5;
        } else if (l % 4 == 0) {
            ss << ":):)"s;
            l -= 4;
        } else if (l % 3 == 0) {
            ss << ":-)"s;
            l -= 3;
        } else if (l % 2 == 0) {
            ss << ":)"s;
            l -= 2;
        } else if (l % 2 != 0) {
            ss << ":-)";
            l -= 3;
        }
    }
    return ss.str();
}

auto s2w(string s) -> width_t { return static_cast<width_t>(std::stoll(s)); }

auto main(int argc, char** argv) -> int
{
    using docopt::docopt;
    using docopt::value;
    using std::cout;
    using std::endl;
    using std::exception;
    using std::map;

    auto width = DEFAULT_WIDTH;

    map<string, value> args = docopt(USAGE, { argv + 1, argv + argc }, true, versionString);

    for (auto const& arg : args) {
        if (arg.first == "<width>"s) {
            try {
                width = s2w(arg.second.asString());
            } catch (exception& e) {
                cout << "Could not convert \""s << arg.second.asString()
                     << "\" to a number, got error: "s << e.what() << endl;
                return 1;
            }
        }
    }

    for (auto i = width_t{ 1 }; i <= width; i++) {
        cout << smiley_line(i) << endl;
    }

    return 0;
}
