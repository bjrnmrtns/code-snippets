#include <thread>
#include <iostream>
#include <stlab/concurrency/future.hpp>
#include <stlab/concurrency/main_executor.hpp>
#include <stlab/concurrency/default_executor.hpp>


int main() {
    auto x = stlab::async(stlab::default_executor, [] { return 42; });
    auto res = x.then([](int x) { std::cout << x << "\n"; });
    while(!res.get_try()) { std::this_thread::sleep_for(std::chrono::milliseconds(1)); }
    return 0;
}
