#include <stlab/concurrency/channel.hpp>
#include <stlab/concurrency/default_executor.hpp>
#include <vector>
#include <thread>
#include <iostream>
#include <numeric>
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
    auto result = receive | [](Image image) { return ImagePlus3(image); }
                          | [](Image image) { return std::accumulate(std::cbegin(image.data), std::cend(image.data), 0); }
                          | [](int x) { std::cout << x << "\n"; };

    receive.set_ready();

    // Do one calculation using the processing tree
    Image image;
    image.data = { 0, 1, 2, 3, 4 };
    Image image2;
    image2.data = { 0, 0, 0 };
    send(image);
    send(image2);

    std::this_thread::sleep_for(std::chrono::milliseconds(100));

    return 0;
}
