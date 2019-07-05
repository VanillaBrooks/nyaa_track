# nyaa_track
anime torrent swarm tracker


## Building 

You will need a copy of the rust compiler which can be downloaded [here](https://www.rust-lang.org/learn/get-started).

clone the repo:

```https://github.com/VanillaBrooks/nyaa_track```

open directory

``` cd nyaa_track```

build & run with compiler optimizations

```cargo run --release```

## postgresql

### Configuration Details

a copy of postgres is required to be running on localhost. 
The default configuration is 

port: ```5432```

user: ```postgres```

password: ```pass```

database name: ```nyaa```

These default parameters can be changed by editing [this portion](https://github.com/VanillaBrooks/nyaa_track/blob/async/src/database/connection.rs#L27-L28) to any setup.

### Importing tables

The following command will import `database.sql` and setup tables required by the binary:

```psql -U <username> -d <database name> -a -f database.sql```

where ```database.sql``` is located in ```src/database/database.sql```
