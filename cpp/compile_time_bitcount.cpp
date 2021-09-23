#include <iostream>
#include <cstdio>

template<class T, T... Ints>
struct integer_sequence {};

template<size_t... Ints>
using index_sequence = integer_sequence<size_t, Ints...>;

template<size_t N, size_t... Ints>
struct index_sequence_helper {
	typedef typename index_sequence_helper<N - 1, N - 1, Ints...>::type type;
};

template<size_t... Ints>
struct index_sequence_helper<0, Ints...> {
	typedef index_sequence<Ints...> type;
};

template<size_t N>
using make_index_sequence = typename index_sequence_helper<N>::type;

constexpr int count_bits(unsigned char value) {
	if (value == 0)
		return 0;
	else
		return (value & 1) + count_bits(value >> 1);
}

template<size_t... V>
struct bit_count_t {
    unsigned char count[sizeof...(V)] = {
        static_cast<unsigned char> (count_bits(V))...
    };
};

template<size_t... V>
constexpr bit_count_t<V...> get_bit_count(index_sequence<V...>) {
    return bit_count_t<V...> ();
}

auto bit_count = get_bit_count(make_index_sequence<256>());

int main() {
    printf("%d", bit_count.count[10]);
    return 0;
}
