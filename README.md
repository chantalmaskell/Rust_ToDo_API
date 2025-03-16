# Rust To-Do List

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