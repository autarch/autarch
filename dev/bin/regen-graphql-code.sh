#!/bin/bash

set -e
set -x

graphql-client generate \
               --schema-path ./graphql/github.schema.graphql \
               --custom-scalars-module crate::gql_types \
               --output-directory ./src/ \
               --response-derives Debug \
               ./graphql/github_queries.graphql
