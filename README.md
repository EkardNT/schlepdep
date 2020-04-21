# schlepdep

"We'll do the schlepping so you can do the depping."

"Schlep less, deploy more"

Safety-focused deployment orchestration software with both stateful and stateless application support.

TODO:
- logging with slog
- decide on API shape (methods and path layout)
- implement the dispatch API
- shutdown with graceful draining
- load testing

On DispatchCommands:
- Write commands
- Check for poll

On ReceiveCommands
- Write poll
- Check for work

How infrastructure works:
- Some things need to be set up manually per account:
    - Route53 hosted zone creation
    - ACM certificate.
