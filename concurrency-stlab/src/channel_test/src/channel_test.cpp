#include <stlab/concurrency/channel.hpp>
#include <stlab/concurrency/default_executor.hpp>
#include <vector>
#include <thread>
#include <iostream>
#include <numeric>
#include <atomic>
#include <algorithm>

namespace {
    struct Image {
        std::vector<unsigned char> data;
    };

    auto ImagePlus3(Image image) {
        Image output;
        std::transform(std::cbegin(image.data), std::cend(image.data), std::back_inserter(output.data),
                [](unsigned char p) { return p + 3; });
        return output;
    }
}

int main() {
    stlab::sender<Image> send;
    stlab::receiver<Image> receive;
    std::tie(send, receive) = stlab::channel<Image>(stlab::default_executor);

    // Setup the processing tree.
    std::atomic_int v {0};
    auto result = receive | [](Image image) { return ImagePlus3(image); }
                          | [](Image image) { return std::accumulate(std::cbegin(image.data), std::cend(image.data), 0); }
                          | [&v](int x) { v = x; };

    receive.set_ready();

    // Do one calculation using the processing tree
    Image image;
    image.data = { 0, 1, 2, 3, 4 };
    send(image);

    while(v == 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }

    std::cout << v << "\n"; // 25

    return 0;
}
