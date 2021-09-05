#include <iostream>
#include <array>

constexpr bool isPrime(int x) {
    if(x == 1)
        return false;
    
    for (int i = 2; i * i <= x; i++) {
        if (x % i == 0)
            return false;
    }
    return true;
}

template<size_t N>
constexpr std::array<int, N + 1> get_prime_seq() {
    std::array<bool, N + 1> check{};
    check.fill(true);
    int tot = 0;
    std::array<int, N + 1> primes{};
    for (int i = 2; i <= N; i++) {
        if (check[i])
            primes[tot++] = i;
        for (int j = 2; j * i <= N; j++)
            check[j * i] = false;
    }
    return primes;
}

auto prime_seq = get_prime_seq<1000>();

int main() {
    for (const auto &x : prime_seq) {
        if (x == 0)
            break;
        std::cout << x << " ";
    }
    return 0;
}