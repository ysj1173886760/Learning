#include <SFML/Graphics.hpp>
#include <iostream>

#include "player.h"
#include "resource_holder.h"

int main() {
    sf::RenderWindow window(sf::VideoMode(1024, 768), "test sprite");

    ResourceHolder<sf::Texture, int> textureHolder;
    textureHolder.load(1, "../assets/survivor-idle_rifle_0.png");

    Player player(&window);
    if (!player.setTexture(textureHolder.get(1)))
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