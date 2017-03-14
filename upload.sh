#!/usr/bin/env bash
read -p "Server: " server
read -p "Username: " username
read -p "Directory: " directory

#rsync -r --info=progress2 public/* $username@$server:$directory
rsync -r public/* $username@$server:$directory
