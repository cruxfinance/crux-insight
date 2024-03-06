# crux-insight

A service that connects to an ergo node and indexes the full history into a
postgres database. Once done syncing it uses zeromq to listen to new blocks
coming in and index them near real-time.

# Env variables

| Env var           | Description                 | Example      |
| ----------------- | --------------------------- | ------------ |
| POSTGRES_USER     | Postgres user of the db     | postgres     |
| POSTGRES_PASSWORD | Postgres password of the db | postgres     |
| POSTGRES_HOST     | Postgres host of the db     | 192.168.1.23 |
| POSTGRES_PORT     | Postgres port of the db     | 5432         |
| POSTGRES_DB       | Postgres db name of the db  | crux_insight |

# local.toml

Adjust the local.toml.example file and save it as local.toml

| config var | Description                                        | Example                    |
| ---------- | -------------------------------------------------- | -------------------------- |
| url        | Full url including port to indexed node            | "http://192.168.1.23:9053" |
| zmq_url    | Full url including port to ergo node zmq publisher | "tcp://192.168.1.23:9060"  |
| pubsubport | Port crux insight publishes indexing events on     | 8765                       |
