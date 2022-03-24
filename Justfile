set shell := ["/bin/bash", "-c"]

run:
  cargo watch -x 'run'

test:
  for x in $(seq 5); do \
    curl "localhost:8080/user/123" & \
  done
