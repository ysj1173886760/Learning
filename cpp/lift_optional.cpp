#include <iostream>
#include <utility>
#include <optional>

template <typename T>
constexpr bool has_value(const std::optional<T> &x) noexcept {
    return x.has_value();
}

template <typename T, typename... Args>
constexpr bool has_value(const std::optional<T> &x, const std::optional<Args>&... other) noexcept {
    return x.has_value() && has_value(other...);
}

template <typename F>
auto lift_optional(F &&f) -> decltype(auto) {
    return [f = std::forward<F>(f)](auto&&... args) {
        typedef std::decay_t<decltype(f(std::forward<decltype(args)>(args).value()...))> result_type;
        if (has_value(args...)) {
            return std::optional<result_type>(f(std::forward<decltype(args)>(args).value()...));
        } else {
            return std::optional<result_type>();
        }
    };
}

template<typename T>
std::ostream& operator<<(std::ostream &os, const std::optional<T> &x) {
    if (x.has_value()) {
        os << *x;
    } else {
        os << "nothing";
    }
    return os;
}

int add(int x, int y) {
    return x + y;
}

int main() {
    auto new_add = lift_optional(add);
    std::cout << new_add(std::make_optional(1), std::make_optional(2)) << std::endl;
    std::cout << new_add(std::make_optional(2), std::optional<int>()) << std::endl;
    return 0;
}