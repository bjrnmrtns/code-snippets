#include <thread>
#include <iostream>
#include <string>
#include <stlab/concurrency/future.hpp>
#include <stlab/concurrency/main_executor.hpp>
#include <stlab/concurrency/default_executor.hpp>
#include <utility>

std::pair<int, bool> someAlgo(int x)
{
    return std::make_pair(x, x == 3);
}

int main() {
    auto x = stlab::async(stlab::default_executor, [] { return 42; });
    auto res = x.then([](int x) { std::cout << x << "\n"; });
    while(!res.get_try()) { std::this_thread::sleep_for(std::chrono::milliseconds(1)); }

    auto arg1 = stlab::async(stlab::default_executor, [] { return 46; });
    auto arg2 = stlab::async(stlab::default_executor, [] { return std::string("Result is: "); });

    auto result = stlab::when_all(stlab::default_executor, [](int result, std::string text) {
            std::cout << text << result << "\n";
    }, arg1, arg2);
    while(!result.get_try()) { std::this_thread::sleep_for(std::chrono::milliseconds(1)); }

    auto someResult = stlab::async(stlab::default_executor, [] { return someAlgo(3);});

    return 0;
}

