# json-grep (WIP)

json-grep (jg) is a binary for searching through json entries in one or multiple files iteratively and recursively, by using a json pattern consisting multiple regexes as query. For example:

```bash
cat numbers.txt
[
    {
        "id": 2000,
        "phone": 111,
        "attributes":{
            "gender": "male"
        }
    },
    {
        "id": 2001,
        "phone": 113,
        "attributes":{
            "gender:": "female"
        }
    },
    {
        "hidden":{
            "id": 2002,
            "phone": 114,
            "attributes":{
                "gender:": "female"
            }
        }
    }
]

cat numbers.txt | jg "{\"id\":\"^2.*$\",\"attr.*\":{\"gender\":\"female\"}}"
[
    {
        "id": 2001,
        "phone": 113,
        "attributes":{
            "gender:": "female"
        }
    },
    {
        "id": 2002,
        "phone": 114,
        "attributes":{
            "gender:": "female"
        }
    }
]
```

It's a tool designed for filtering and searching entries in json files of interest with a hard and maybe unclear query pattern, in a more dedicated and elegant way than `jq`.

`jg` is written in `Rust` programming language, and with a limited parallelism support, it should be able to handle most of the json files in a reasonable time, and we will improve the performance overtime.

Right now the program is still being developed, so the readme and related documents are more like some development log of my own, to record some basic ideas and goals which I thought before.

## 1. Why use json-grep

There are already several text-processing tools, or something that can deal with JSON texts and files, but things like `grep` or `ripgrep` failed to treat JSON in a JSON way, instead they treated them like plain text - which led to some problem, or `jq`, which is surely the most convenient tool for JSON transformation and processing, but unfortunately, they don't have a complete grep support in JSON style, since it's not used in that way, and so, the `json-grep` was come out in my mind.
