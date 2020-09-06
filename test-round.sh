#!/bin/bash

# Load test execution
loadtest(){
  # navigate to tests folder
  cd ../loadtest/
  # start load test
  npm run $1
}

take_a_break(){
  # wait for 30 sec
  echo "sleep...$1 sec"
  sleep $1
}

start_api(){
  docker-compose up -d
  # wait
  take_a_break 25
}

stop_api(){
  docker-compose down --volumes
  # wait
  take_a_break 5
}

# ---------------------
# ACTIX Deadpool test
# start actix api
cd actix-dp
# start api with docker compose
start_api
# run load test
loadtest test-actix-dp
# go back to api folder
cd ../actix-dp
# stop api and remove volumes
stop_api

# ---------------------
# ACTIX GraphQL test
# start actix api
cd ../actix-gql
# start api with docker compose
start_api
# run load test
loadtest test-actix-gql
# go back to api folder
cd ../actix-gql
# stop api and remove volumes
stop_api

# ---------------------
# ACTIX v2 Deadpool test
# start actix api
cd actix-v2-dp
# start api with docker compose
start_api
# run load test
loadtest test-actix-v2-dp
# go back to api folder
cd ../actix-v2-dp
# stop api and remove volumes
stop_api