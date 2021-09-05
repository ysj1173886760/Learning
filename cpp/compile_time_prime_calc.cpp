#include <iostream>
#include <cstdio>

constexpr bool isPrime(int x) {
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

template<bool cond, typename Then, typename Else>
struct If;

template<typename Then, typename Else>
struct If<true, Then, Else> {
    typedef Then type;
};

template<typename Then, typename Else>
struct If<false, Then, Else> {
    typedef Else type;
};

template<size_t N, size_t... Ints>
struct prime_calc_helper {
    typedef 
        typename If<isPrime(N), 
                    typename prime_calc_helper<N - 1, N, Ints...>::type,
                    typename prime_calc_helper<N - 1, Ints...>::type>::type 
        type;
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

auto prime_seq = get_prime_seq(make_prime_sequence<5>());

int main() {
    std::cout << prime_seq.count[1];
    return 0;
}
