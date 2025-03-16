# Rust To-Do List

Simple to-do list in Rust using Warp (http server), Serde (serialisation/deserialisation), and an in-memory database.

### Run Server (hot reload)
```
cargo-watch -q -c -w src/ -x run
```

### Endpoints
GET - Health <br>
```
/api/healthcheck
```

GET - To-do by ID <br>
```
/api/todos{id}
```

POST - Add to-do task<br>
```
/api/todos
```

PATCH - Update task<br>
```
/api/todos/{id}
```

GET - Get all to-do tasks<br>
```
/api/todos/?page={num}&limit={num}
```
