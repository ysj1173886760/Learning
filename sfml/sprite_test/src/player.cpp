#include <cmath>
#include <iostream>
#include "player.h"

const float COS45 = 0.707106f;

void Player::update(const sf::Time &deltaTime) {
    sf::Vector2i mousePos = sf::Mouse::getPosition(*_window);
    sf::Vector2f currentPos = _shape.getPosition();

    auto deltaX = mousePos.x - currentPos.x;
    auto deltaY = mousePos.y - currentPos.y;

    auto angle = atan2(deltaY, deltaX) / M_PI * 180;
    _shape.setRotation(angle);

    const int factor = 10;
    sf::Vector2f velocity(0, 0);

    if (sf::Keyboard::isKeyPressed(sf::Keyboard::W))
        velocity.y = -1;
    else if (sf::Keyboard::isKeyPressed(sf::Keyboard::S))
        velocity.y = 1;
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::A))
        velocity.x = -1;
    else if (sf::Keyboard::isKeyPressed(sf::Keyboard::D))
        velocity.x = 1;
    
    float norm = 1.0f;
    if (velocity.y && velocity.x)
        norm = COS45;
    double dt = deltaTime.asMicroseconds();
    _shape.move(sf::Vector2f(velocity.x * norm * dt * _speed, velocity.y * norm * dt * _speed));
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