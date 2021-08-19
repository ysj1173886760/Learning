#include <cmath>
#include <iostream>
#include "player.h"

void Player::update(const sf::Time &deltaTime) {
    sf::Vector2i mousePos = sf::Mouse::getPosition(*_window);
    sf::Vector2f currentPos = _shape.getPosition();

    auto deltaX = mousePos.x - currentPos.x;
    auto deltaY = mousePos.y - currentPos.y;

    auto angle = atan2(deltaY, deltaX) / M_PI * 180;
    _shape.setRotation(angle);
}

bool Player::loadTexture() {
    if (texture.loadFromFile("../assets/survivor-idle_rifle_0.png")) {
        _shape.setTexture(&texture);
        return true;
    }
    return false;
}

void Player::draw(sf::RenderTarget &target, sf::RenderStates states) const {
    target.draw(_shape, states);
}