//TODO: Resolve dependencies automatically on start 
//TODO: Threads?

consider a min and max thread number, and dynamically resize between them, 
this way we dont use more resources than we need, 
cant use more than we have, and always have some ready to go

Details Rails server command 

// => Booting Puma
// => Rails 7.0.6 application starting in development 
// => Run `bin/rails server --help` for more startup options
// Puma starting in single mode...
// * Puma version: 5.6.6 (ruby 3.2.2-p53) ("Birdie's Version")
// *  Min threads: 5
// *  Max threads: 5
// *  Environment: development
// *          PID: 9361
// * Listening on http://127.0.0.1:3000
// * Listening on http://[::1]:3000
// Use Ctrl-C to stop
// ^C- Gracefully stopping, waiting for requests to finish
// === puma shutdown: 2023-07-14 14:13:41 -0500 ===
// - Goodbye!
// Exiting

// ❯ rails server --help
// Usage:
//   rails server -u [thin/puma/webrick] [options]
//
// Options:
//   -e, [--environment=ENVIRONMENT]              # Specifies the environment to run this server under (test/development/production).
//   -p, [--port=port]                            # Runs Rails on the specified port - defaults to 3000.
//   -b, [--binding=IP]                           # Binds Rails to the specified IP - defaults to 'localhost' in development and '0.0.0.0' in other environments'.
//   -c, [--config=file]                          # Uses a custom rackup configuration.
//                                                # Default: config.ru
//   -d, [--daemon], [--no-daemon]                # Runs server as a Daemon.
//   -u, [--using=name]                           # Specifies the Rack server used to run the application (thin/puma/webrick).
//   -P, [--pid=PID]                              # Specifies the PID file - defaults to tmp/pids/server.pid.
//   -C, [--dev-caching], [--no-dev-caching]      # Specifies whether to perform caching in development.
//       [--early-hints], [--no-early-hints]      # Enables HTTP/2 early hints.
//       [--log-to-stdout], [--no-log-to-stdout]  # Whether to log to stdout. Enabled by default in development when not daemonized.



