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

