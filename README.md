# newsletter-backend-template
Companion repo for building solid rust backends

## Acknowledgments
This repository is a source of knowledge and hands on distributed systems. 

It is the art of refactoring in order to make an aplication scalable.

[zero-to-production](https://github.com/LukeMathWalker/zero-to-production)
</br>
[book](https://www.zero2prod.com/index.html?country=Netherlands&discount_code=VAT20)

## Learnings
- Rust api
- Rust tooling
- Monolithic backend
- Idempotency pattern
- Distributed transactions

## Best effort delivery - Highlights
- Use of idepontency keys to guarantee safe retries (the caller has no way to observe if a request has been sent to the server once or multiple times. )
- Use of distributed transactions to guarantee consistency in concurrent requests
- Use of delivery workers for async processing
- Use of distributed transactions using forward recovery to ensure sunchronization of the async workers

Great explanation [here](https://www.lpalmieri.com/posts/idempotency/#take-home)

[This commit](https://github.com/PanGan21/newsletter-backend-template/commit/c4dc6a5a59dcc6ee07435b41e2dfc793cd6360b5) includes the refactor required to 
ensure best effort delivery for a cloud native application.
