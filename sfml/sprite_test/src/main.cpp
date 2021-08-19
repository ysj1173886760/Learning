#include <SFML/Graphics.hpp>
#include <iostream>

int main() {
    sf::RenderWindow window(sf::VideoMode(1024, 768), "test sprite");

    sf::Texture texture;
    if (!(texture.loadFromFile("../assets/survivor-idle_rifle_0.png"))) {
        std::cerr << "warning: can not load texture" << std::endl;
        return 0;
    }
    
    sf::Vector2u textureSize = texture.getSize();
    sf::RectangleShape rect(sf::Vector2f(textureSize.x / 2, textureSize.y / 2));
    rect.setTexture(&texture);

    while(window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::EventType::Closed)
                window.close();
        }

        window.clear();
        window.draw(rect);
        window.display();
    }

    return 0;
}