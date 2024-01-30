# VARiation

A Simple CLI application that stores, deletes or presents variables based on the current working directory or path provided.


```
+-----------+       +---------------+          +-------+            +-----------+
|  Options  |  -->  |    Project    |   --+--> | print |     --->   |  Display  |
+-----------+       | Configuration |     |    +-------+            +-----------+
                    +---------------+     |
                                          |
                                          |    +-----+              +--------+
                                          +--> | add |       --->   |  Save  |
                                          |    +-----+              +--------+
                                          |
                                          |
                                          |    +--------+           +----------+
                                          +--> | remove |    --->   |  Output  |
                                               +--------+           +----------+
```

## Dependencies 
- **Clap**  { CLI Parser }
- **Anyhow** { Error Handling }
- **Collection** { Hashmap }
- **Serde** { Serialization }


# How to Use

-  cargo build { to initialize}

-  cargo run -- --config /path/to/folder/something.json OPTIONS ARGS


### Possible Options with args :

1. print all the key value pairs
```
cargo run -- --config /path/to/folder/something.json 
```

2. add a key value pair
```
cargo run -- --config /path/to/folder/something.json add key value
```

3. remove a key value pair
```
cargo run -- --config /path/to/folder/something.json remove key
```

4. print value of a key
```
cargo run -- --config /path/to/folder/something.json key
```

