---
title: "fitness tracker"
date: 2025-12-30
description: "personal fitness tracker using hugo and python"
tags: ["python", "hugo", "fitness", "automation"]
---

I've been tracking my workouts[^1] for a while now, running data syncs from
[strava](https://www.strava.com/), lifting logs live in a google sheet.
The natural instinct would be to build a  web app with a database, user auth,
real-time updates. But I went a different direction: a static site that rebuilds
itself.

## why static?

- i'm cheap/free hosting (for now)
- no server to maintain
- deploying the site is `git push`
- faster to iterate on visualizations

## why _not_ static?

- no interactivity (no one can comment)
- i create my own shortcuts and visualizations

The analyzer does the heavy lifting (pun intended). It normalizes exercise
names so "flat bb bench press" and "barbell bench" both map to "bench press".
It calculates estimated 1RMs using the [epley formula](https://en.wikipedia.org/wiki/One-repetition_maximum).
It tracks PRs across different rep ranges—strength (1-5 reps),
hypertrophy (6-10), endurance (11+).

The data flows through pure functions. No side effects, no state. Feed it a
list of workouts, get back stats. Feed it a list of strava activities, get back
mileage charts.

## shortcodes

Hugo shortcodes are basically serverless functions that run at build time.
The `{{</* volume-trend */>}}` shortcode reads `site/data/weekly_volume.json`
and renders a chart.js visualization. no javascript framework, no build step
for the frontend. Just template logic and a charting library.

The shortcodes are dumb, they just render what's in the json. All the
calculation happens in python. This separation makes debugging easy. If a chart
looks wrong, either the python is calculating wrong data or the shortcode is
rendering it wrong. Check the json file to know which.

[^1]: I say 'fitness' but it's my workouts currently (running and lifting), but
      I should mention involves nutrition and other health metrics down the line.
