#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <numeric>
#include <functional> // std::multiplies

int main()
{
    std::ifstream data("input.in");
    std::vector<int> signal;
    char input;
    while(data.get(input)){
        signal.push_back(static_cast<int>(input-'0'));
    }
    auto n = signal.size();
    std::vector<std::vector<int>> phase_transform{n, std::vector<int>(n,0)};
    std::vector pattern{0, 1, 0, -1};
    for (auto i = 0; i < n; i++)
    {
        std::generate(phase_transform[i].begin(), phase_transform[i].end(), [i, pattern, j = 1]() mutable {
            int out = pattern[j / (i+1)];
            j++;
            if (j / (i+1) == pattern.size())
            {
                j = 0;
            }
            return out;
        });
    }
    for (auto phase = 0; phase < 100; phase++)
    {
        for (auto i = 0; i < n; i++)
        {
            std::inner_product(phase_transform.begin(), phase_transform.end(), signal.begin(),
                           [signal](std::vector<int> phase_map) {
                               std::vector<int> middle;
                               std::transform(signal.begin(), signal.end(), phase_map.begin() + 1, std::back_inserter(middle),
                                              std::multiplies<int>{});
                               int result = std::accumulate(middle.begin(), middle.end(), 0);
                               return std::abs(result) % 10;
                           });
        }
    }
    std::cout << "fuel: " << std::accumulate(signal.begin(), signal.end(), std::string{}, [](std::string s, int i) {s.append(1,'0'+i);return s; }) << std::endl;
}