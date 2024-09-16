![Logo](docs/hulaak.png)
# Hulaak (who-laak)

Hulaak (Nepali word for post office) is a small personal project I am doing as a PoC for moving data within and between machines. Hulaak works on the concept of a controller stub that controls modules on a machine which can gather data from various sources, and communicate it to modules on the same machines or a different machine. Hulaak modules are [actors](https://en.wikipedia.org/wiki/Actor_model), and Hulaak controllers are synchronized using the [raft consensus algorithm](https://raft.github.io/). This is still a WiP, but the I have a general idea of how the configuration is going to look like.

Looking forward to completing this project!