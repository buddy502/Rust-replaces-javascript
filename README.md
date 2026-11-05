# echo_chamber
Every connection creates an echo and distorts the web page based on time between other connections.

Imagine storing a vector of users time between each connection in seconds ->
[4, 8, 100, 30, 9, 2]

The echo chamber will distort the web page between connections so that every connection is different.
Although not random, a new entry will come back later down the line like an echo. For example, the
user connection at 4 seconds will echo throughout the other users and come back to the user at 100 seconds.

I'm going to be building this using rust, html, and css. Javascript will not be used if I can prevent it.
- Keep track of every user GET request to the html page
- Lock in

I think this is an original idea, at least that's what I'm trying to build.
