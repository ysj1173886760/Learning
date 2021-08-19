#include <SFML/Graphics.hpp>
#include <iostream>
#include "player.h"

int main() {
    sf::RenderWindow window(sf::VideoMode(1024, 768), "test sprite");

    Player player(&window);
    if (!player.loadTexture())
        return 0;

    sf::Clock clock;

    while(window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::EventType::Closed)
                window.close();
        }

        player.update(clock.restart());

        window.clear();
        window.draw(player);
        window.display();
    }

    return 0;
}