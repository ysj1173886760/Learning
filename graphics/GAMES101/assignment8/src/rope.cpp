#include <iostream>
#include <vector>

#include "CGL/vector2D.h"

#include "mass.h"
#include "rope.h"
#include "spring.h"

namespace CGL {

    Rope::Rope(Vector2D start, Vector2D end, int num_nodes, float node_mass, float k, vector<int> pinned_nodes)
    {
        // TODO (Part 1): Create a rope starting at `start`, ending at `end`, and containing `num_nodes` nodes.
        double x_length = (end.x - start.x) / (num_nodes - 1);
        double y_length = (end.y - start.y) / (num_nodes - 1);
        
        Vector2D cur = start;

        for (int i = 0; i < num_nodes; i++) {
            Mass *newNode = new Mass(cur, node_mass, false);

            if (!masses.empty())
                springs.push_back(new Spring(masses.back(), newNode, k));

            masses.push_back(newNode);
            cur.x += x_length;
            cur.y += y_length;
        }
//        Comment-in this part when you implement the constructor
        for (const auto &i : pinned_nodes) {
            masses[i]->pinned = true;
        }
    }

    void Rope::simulateEuler(float delta_t, Vector2D gravity)
    {
        for (auto &s : springs)
        {
            auto dist = (s->m1->position - s->m2->position).norm();
            auto lenDelta = dist - s->rest_length;
            auto dir = (s->m1->position - s->m2->position) / dist;
            s->m1->forces += -1 * s->k * dir * lenDelta;
            s->m2->forces += s->k * dir * lenDelta;
            
            // TODO (Part 2): Use Hooke's law to calculate the force on a node
        }

        for (auto &m : masses)
        {
            if (!m->pinned)
            {
                m->forces += gravity * m->mass;
                auto acce = m->forces / m->mass;
                m->velocity += acce * delta_t;
                m->position += m->velocity * delta_t;
                // TODO (Part 2): Add the force due to gravity, then compute the new velocity and position

                // TODO (Part 2): Add global damping
            }

            // Reset all forces on each mass
            m->forces = Vector2D(0, 0);
        }
    }

    void Rope::simulateVerlet(float delta_t, Vector2D gravity)
    {
        for (auto &s : springs)
        {
            auto dist = (s->m1->position - s->m2->position).norm();
            auto lenDelta = dist - s->rest_length;
            auto dir = (s->m1->position - s->m2->position) / dist;
            s->m1->forces += -1 * s->k * dir * lenDelta;
            s->m2->forces += s->k * dir * lenDelta;
            // TODO (Part 3): Simulate one timestep of the rope using explicit Verlet （solving constraints)
        }

        for (auto &m : masses)
        {
            if (!m->pinned)
            {
                Vector2D temp_position = m->position;
                // TODO (Part 3.1): Set the new position of the rope mass
                
                m->forces += gravity * m->mass;
                auto acce = m->forces / m->mass;

                float factor = 0.00005;
                m->position = temp_position + (1 - factor) * (temp_position - m->last_position) + acce * delta_t * delta_t;
                m->last_position = temp_position;
                // TODO (Part 4): Add global Verlet damping
            }
            m->forces = Vector2D(0, 0);
        }
    }
}
