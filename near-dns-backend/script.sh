#!/bin/bash

DOMAIN=test
TOKEN=$(pass duckdns/token)

echo curl "https://www.duckdns.org/update?domains=${DOMAIN}&token=${TOKEN}&ip=${A}&ipv6=${AAAA}&verbose=true&clear=true"