#!/bin/bash

{
  echo "put user alice"
  echo "get user"
  echo "delete user"
  echo "get user"
  echo "exit"
} | nc 127.0.0.1 6379
