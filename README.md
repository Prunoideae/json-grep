# json-grep (WIP)

json-grep is a binary for searching through json entries in one or multiple files iteratively and recursively, by using a json pattern consisting multiple regexes as query. For example:

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

cat numbers.txt | jg {"id":"^2.*$","attr.*":{"gender":"female"}}
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
