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

<br>
<br>
GET - To-do by ID <br>
```
/api/todos{id}
```

<br>
<br>
POST - Add to-do task<br>
```
/api/todos
```

<br>
<br>
PATCH - Update task<br>
```
/api/todos/{id}
```

<br>
<br>
GET - Get all to-do tasks<br>
```
/api/todos/?page={num}&limit={num}
```