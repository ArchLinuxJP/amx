```sh
# token
$ curl -XGET "https://localhost:8448/_matrix/client/r0/login"
$ curl -XPOST -d '{"type":"m.login.password", "user":"example", "password":"wordpass"}' "https://localhost:8448/_matrix/client/r0/login"
```
