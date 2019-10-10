WHAT

An API-only webservice that creates and updates users in a postgres database, possibly with a latitude/longitude calculation before saving.

WHY

This is easy to build in any other language. However this implementation uses Rust and is aimed to be an order of magnitude faster, and (slightly) more reliable than your standard vanilla implementation.

HOW

Add credentials of your local postgres install in `.env`

    $ echo DATABASE_URL=postgres://postgres:@localhost/rust_pure_api > .env
    $ diesel migration run

You might have to create a database `rust_pure_api` manually

    $ cargo run

Try creating a user, in another Terminal tab:

```
    curl http://127.0.0.1:8088/users \                                                                                                
            -H "Content-Type: application/json" \
            -d '{"first_name": "Henry", last_name: "Dubo", player_number: 201172, "address1": "5208 Av Parc", city: "Montreal", country: "Canada"}'
```

There should be a new row in users table.

Try deleting a user:

```
    curl -X DELETE http://127.0.0.1:8088/users/1 \
        -H "Content-Type: application/json"
```

There should be one less row in users table.


NEXT

* `to_async` + diesel ? https://github.com/diesel-rs/diesel/issues/399
* how does this deal with concurrent requests?
* Use an in-memory store such as redis instead of postgres and then benchmark if you can easily get 1Mops. This would be similar to a trading platform receiving buy/sell orders.
* Tests