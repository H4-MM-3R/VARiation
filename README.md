# VARiation

A Simple CLI application that stores, deletes or presents variables based on the current working directory or path provided.


+-----------+       +---------------+          +-------+             +-----------+
|  Options  |  -->  |    Project    |   --+--> | print |     --->    |  Display  |
+-----------+       | Configuration |     |    +-------+             +-----------+
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

## Dependencies 
- **Clap**  { CLI Parser }
- **Anyhow** { Error Handling }
- **Collection** { Hashmap }
- **Serde** { Serialization }


## Idea

/foo/bar/baz/path/to/folder variance

### look for entries with

/foo/bar/baz/path/to/folder
/foo/bar/baz/path/to
/foo/bar/baz/path
/foo/bar/baz
/foo/bar
/foo
/

Merge each of the value set together from right to left,
( Left being Least Priority )

### This is Same for

/foo/bar/baz/path/to/folder variance foo

# look for entries with
/foo/bar/baz/path/to/folder # no foo
/foo/bar/baz/path/to # no foo
/foo/bar/baz/path # found foo < return value now


# Implementation
                           +-------------+
1. Creating CLI options    | CLI Options | -->  { Parse from CLI }
                           +-------------+


