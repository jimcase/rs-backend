[![Unit tests](https://github.com/jimcase/rs-backend/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/jimcase/rs-backend/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/jimase/rs-backend/branch/main/graph/badge.svg)](https://codecov.io/gh/jimase/rs-backend)

# RS Backend
## Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
~/.cargo/bin/rustc --version
vim ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"
```


#### Create database
```bash
    sqlite3 database.db
```

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);

.tables

.quit

sqlite3 test.db "SELECT * FROM usuarios;"
```

#### Run 
```bash
    cargo install cargo-watch
    cargo watch -x run
```

Create user:
```bash
curl -X POST http://localhost:8080/users \
-H "Content-Type: application/json" \
-d '{"nombre": "Juan Pérez", "email": "juan.perez@example.com"}'
```

Get user:
```bash
curl -X GET http://localhost:8080/users/1
```

Update user:
```bash
curl -X PUT http://localhost:8080/users/2 \    
-H "Content-Type: application/json" \
-d '{"nombre": "Juan Pérez Actualizado", "email": "juan.perez.actualizado@example.com"}'
```

Delete user
```bash
curl -X DELETE http://localhost:8080/usuarios/1
```bash