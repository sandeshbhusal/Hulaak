![Logo](docs/hulaak.png)
# Hulaak (who-laak)

Hulaak (Nepali word for post office) is a small personal project I am doing as a PoC for moving data within and between machines. Hulaak works on the concept of a controller stub that controls modules on a machine which can gather data from various sources, and communicate it to modules on the same machines or a different machine. Hulaak modules are [actors](https://en.wikipedia.org/wiki/Actor_model) that can talk to local modules, and the controller is also an Actor, that can talk to both local modules, and other remote controllers.

## Example:

In order to run this program yourself, compile it, add a `-c` switch as an argument to a configuration file (toml) like this:

```toml
[modules]
[modules.echo_file]
module_type = "echo"

[modules.udp_sock_list]
module_type = "udpsocketlistener"
address = "0.0.0.0"
port = 8080
buffer_size = 1024

[modules.tcp_socket_check]
module_type = "tcpsocketlistener"
address = "0.0.0.0"
port = 8081
buffer_size = 4096

[routes]
[routes.simple_echo_from_file]
from = { Multiple = ["udp_sock_list", "tcp_socket_check"]}
to = { Single = "echo_file"}
```

And hulaak will begin listening to messages on the specified UDP socket, and echo it out to the terminal. The capabilities (and modules included) in hulaak are increasing by the day, so stay tuned!
