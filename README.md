# Postmortem

## Initial Concept 
I chose to base my project on Chapter 2, Challenge 4 from Brathwaite’s and Ian Schreiber’s Challenges for Game Designers. The challenge consisted of making a game where players must gather or pick up something. While coming up with the idea for my game I realized that I had an intense desire to learn the Rust programming language. So I decided that I'd make a console-based game that plays like a board game.

As of the last week, I've been watching a lot of videos about the Gold Shaw Farm in Vermont. They regularly post update videos on their YouTube channel about life on the farm. Even though they primarily raise poultry, I felt that I could tie in my current interest in farming with a basic console-style game.

This resulted in Garden Bash, a turn-based carrot gathering game.

The basic instructions are as follows...

Each player has a set number of turns to harvest the most carrots possible. Move atop a carrot to pick it. Move near a player and use the "steal" command to take at most five carrots from them.

Right now, I couldnt get the stealing functionality to work but movement and collecting does work which is what was the most important thing for me.

## Concepts
The game has a few core mechanics. It's most important mechanic for this challenge, however, was the gathering mechanic.

Players can move to the carrots on the map to pick them up and earn points.

