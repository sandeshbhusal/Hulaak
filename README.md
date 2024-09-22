![Logo](docs/hulaak.png)
# Hulaak (who-laak)

Hulaak (Nepali word for post office) is a small personal project I am doing as a PoC for moving data within and between machines. Hulaak works on the concept of a controller stub that controls modules on a machine which can gather data from various sources, and communicate it to modules on the same machines or a different machine. Hulaak modules are [actors](https://en.wikipedia.org/wiki/Actor_model) that can talk to local modules, and the controller is also an Actor, that can talk to both local modules, and other remote controllers.