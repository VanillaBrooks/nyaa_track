# nyaa_track
anime torrent swarm tracker

## About 

This is a utility for tracking torrent statistics. Currently it pulls information on torrent seeds, total downloads, and current downloads. While this utility currently focuses on [nyaa](https://github.com/nyaadevs/nyaa/tree/90607d6993316917676ec19dd6812c25184850de), it can quickly be configured for a variety of other trackers that support RSS feeds with relative ease. 

In order to be tracked, a torrent's info hash is parsed from the trackers RSS feed and fed into an event loop that will asynchronously pull its data. The default configuration for dropping a torrent from the event loop is when the torrent has less than 100 seeds and is older than seven days.

On startup, `nyaa_track` will pull all previous torrent details from postgres if they are not still in the rss feed. UDP trackers are a future goal but not a priority as nyaa runs on https.

`nyaa_track` only supports https trackers currently. 

## Startup

NOTE: if you are not on windows you must have openssl installed.

### Releases

You can download a binary and config file from the [releases section](https://github.com/VanillaBrooks/nyaa_track/releases)

### Building 
You will need a copy of the rust compiler which can be downloaded [here](https://www.rust-lang.org/learn/get-started).

clone and open the repo:

```git clone https://github.com/VanillaBrooks/nyaa_track && cd nyaa_track```

build & run with compiler optimizations

```cargo run --release```


## postgresql

### Configuration Details

Configuration can be done with the `config.json` found at the root directory of the project. Currently the database is expected to be hosted at `localhost`. Current configuration accepts changes to `port`, `database_name`, `username` and `password`. 

A sample directory for a would be:

```
tracker/
...nyaa_track.exe
...config.json
... temp/
```

Note that the `temp` folder is created by `nyaa_track.exe` to temporarily store downloaded `.xml` files. 

### Importing tables from scratch

The following command will import `database.sql` and setup tables required by the binary in an already created `database_name`:

```psql -U username -d database_name -a -f database.sql```

where ```database.sql``` is located in ```src/database/database.sql```

### Importing previous data

with an already created database named `database_name` you can run the command:

`psql -U username -d database_name < infile`

Reading a compressed database file can be done with:

`gunzip -c filenam.gz | psql -U username -d database_name`


### dumping database files

You can dump data from `database_name` with:

`pg_dump -U username -d database_name > outfile`

or with compression:

`pg_dump -U username -d database_name | gzip > filename.gz`


## Storage

By my estimations the tracker functioning at 10 requests / second is projected to store less than 2 GB / month of data. 


## Async Await

Before async await (future 0.1):

```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 JSON                    1            6            6            0            0
 Markdown                1           83           83            0            0
 Python                  2          226          179           16           31
 Rust                   17         2519         1876          260          383
 SQL                     2          131           89           11           31
 TOML                    1           41           28            1           12
-------------------------------------------------------------------------------
 Total                  24         3006         2261          288          457
-------------------------------------------------------------------------------
```