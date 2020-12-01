#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <numeric>

auto increasing(std::string pass) -> bool
{
    return std::adjacent_find(pass.cbegin(), pass.cend(), [](char a, char b) {
               return a > b;
           }) == pass.cend();
}

auto double_digit(std::string pass)
{
    return std::adjacent_find(pass.cbegin(), pass.cend(), [](char a, char b) {
               return a == b;
           }) != pass.cend();
}
auto double_digit_not_more(std::string pass)
{
    auto it = pass.cbegin();
    char prev{*it};
    it++;
    bool is_double{false};
    while (it != pass.cend())
    {
        if (prev != *it && is_double)
        {
            return is_double;
        }
        if (prev == *it && is_double)
        {
            is_double = false;
            while (prev == *it)
            {
                it++;
                if (it == pass.cend())
                {
                    return is_double;
                }
            }
        }
        if (prev == *it)
        {
            is_double = true;
        }
        prev = *it;
        it++;
    }
    return is_double;
}

auto meet_critera(int pass) -> bool
{
    return double_digit_not_more(std::to_string(pass)) && increasing(std::to_string(pass));
}

int main()
{
    int test_pass{125730};
    int possible_pas{0};
    while (test_pass < 579381)
    {
        if (meet_critera(test_pass))
        {
            possible_pas++;
        }
        test_pass++;
    }
    std::cout << "number of pos_pas: " << possible_pas << std::endl;

}