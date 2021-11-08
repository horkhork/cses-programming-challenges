```
╰─ while true ; do inotifywait -e modify -r src ; echo "3" | cargo run ; done
```
