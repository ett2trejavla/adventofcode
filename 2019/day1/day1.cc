#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <numeric>

long long req_fuel(long long weight)
{
    long long total_fuel = weight / 3 - 2;
    long long fuel_i = total_fuel;
    while (true)
    {
        fuel_i = fuel_i / 3 - 2;
        if (fuel_i <= 0)
        {
            break;
        }
        total_fuel += fuel_i;
    }
    return total_fuel;
}

int main()
{
    std::ifstream data("day1.in");
    std::vector<long long> module_weight(100, 0);
    int i = 0;
    while (data >> module_weight[i])
    {
        i++;
    }
    long long fuel = std::accumulate(module_weight.cbegin(), module_weight.cend(), 0ll,
                                     [](long long acc, long long x) {
                                         return acc + req_fuel(x);
                                     });
    std::cout << "fuel: " << fuel << std::endl;
}