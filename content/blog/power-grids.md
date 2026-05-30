---
title: "power grids and single points of failure"
date: 2025-12-31
description: "a reflection on power grids and system design"
tags: ["electrical-engineering", "comptuer-science", "power-grids"]
---

While helping my family setup a new smart camera, I was read through the
installation instructions where it had a diagram describing what wires were
the "live" (hot), neutral, and earth (ground). It got me thinking about power grids
and what it meant to be grounded again.

- neutral wire: completes the circuit back to the source, normally at 0V
- live (hot) wire: carries the voltage from the source to the load (your home),
  normally at some positive voltage (e.g. 120V or 240V AC)
- earth (ground) wire: safety measure to prevent electric shock, normally at 0V
  - some homes you can see a ground rod (often copper) driven into the earth outside
    the house, connected to the home's electrical system. this helps dissipate
    excess electricity safely into the ground in case of a fault (not cause
    fires or shocks)

I had to look up why we have a neutral wire and came to the (perhaps obvious)
realization that the neutral wire is essential for completing the circuit.
Without it, current wouldn't flow through the load (your appliances) because
there would be no return path to the source. The neutral wire ensures that
the circuit is complete, allowing electricity to flow properly and safely.

This in turn had me better understand why power outages [usually] affect
multiple homes in an area. The power grid is designed with redundancy
and multiple distribution lines to ensure reliability. However, if a main
distribution line or transformer fails, it can lead to outages for all
homes connected to that segment of the grid. This is because the failure
disrupts the flow of electricity to all the homes relying on that particular
infrastructure.

This made me think about single points of failure in systems design. Naively,
I thought why don't we have backup lines for every home? But that would be
impractical and cost-prohibitive. Instead, power grids are designed with
redundancy at higher levels (e.g., multiple transformers, alternative routes)
to minimize the impact of failures with respect to a segment of the grid rather
than individual homes.

This is similar to how we design distributed systems in computer science, where
we aim to eliminate single points of failure by introducing redundancy and
failover mechanisms at critical points in the architecture. My initial thought
was to have every home connected to multiple power lines or sources, and thought
"ah what a poor design by those electrical engineers!" but then realized
that this system actually was designed with minimizing the single points of
failure to a pretty large scale already, e.g. having only a segment of the grid
fail rather than the entire grid. More often than not, electrical engineers
humble me with how little I know about the actual physics and hardware
interactions that represent the real-world rather than the virtual world
of bits and bytes I usually think about.

This was just a reflection I had while setting up a smart camera and talking
about it with the electrical engineer(s) in my family.
