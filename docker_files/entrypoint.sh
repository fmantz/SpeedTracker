#!/bin/sh

# start cron deamon
cron start

# one docker process must run in foreground:
httpd-foreground
