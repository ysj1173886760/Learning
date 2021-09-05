#include <iostream>
#include <cstdio>

constexpr bool isPrime(int x) {
    if(x == 1)
        return false;
    
    for (int i = 2; i * i <= x; i++) {
        if (x % i == 0)
            return false;
    }
    return true;
}

template<class T, T... Ints>
struct integer_sequence {};

template<size_t... Ints>
using index_sequence = integer_sequence<size_t, Ints...>;

template<size_t N, size_t... Ints>
struct prime_calc_helper;

template<size_t N, size_t... Ints>
struct late {
    using type = typename prime_calc_helper<N, Ints...>::type;
};

template<size_t N, size_t... Ints>
struct prime_calc_helper {
    using type = typename std::conditional<isPrime(N), 
                                    late<N - 1, N, Ints...>,
                                    late<N - 1, Ints...>>::type::type;
};

template<size_t... Ints>
struct prime_calc_helper<0, Ints...> {
    typedef index_sequence<Ints...> type;
};

template<size_t N>
using make_prime_sequence = typename prime_calc_helper<N>::type;

template<size_t... Ints>
struct prime_sequence_t {
    int count[sizeof...(Ints)] = {
        Ints...
    };
};

template<size_t... Ints>
constexpr prime_sequence_t<Ints...> get_prime_seq(index_sequence<Ints...>) {
    return prime_sequence_t<Ints...> ();
}

auto prime_seq = get_prime_seq(make_prime_sequence<100>());

int main() {
    for (const auto &x : prime_seq.count) {
        std::cout << x << " ";
    }
    return 0;
}
