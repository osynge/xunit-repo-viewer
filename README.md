# xunit-repo-viewer

## Introduction

The xunit-repo-viewer servers junit/[xunit2](https://xunit.net/) results from a local database. The [xunit-repo](https://github.com/osynge/xunit-repo) updates this database from results provided by the [xunit-repo-client](https://github.com/osynge/xunit-repo-client).

xunit-repo-viewer can be configured with environment variables, configuration files, or command line arguments, in order from lowest to highest precedence. xunit-repo-viewer is expected to be used either on the developers desktop, or as part of a continuous integration, continuous deployment framework such as jenkins or drone.

## Features

* Aggregate all your xunit test results, from multiple projects and files in one web site.
* Browse old and new builds.
* See your test errors, failures, and skipped tests before your test passes.
* Support multi environment builds, eg. parallel builds across operating system and architecture.
* Uses a relational database of your test results, so each build uses less disk space.
* Logging can be traditional line based or json.
* Easy to monitor with prometheus metrics.

### Table of xunit-repo-client configuration

Setting | Type | Environment variable | Configuration parameter | Command line argument
------- | ---- | -------------------- | ----------------------- | ---------------------
Database URL | String | XRV_DATABASE | database_url | --database-url
Migrate database | Boolean | XRV_DATABASE_MIGRATE | database_migrate | --database-migrate --no-database-migrate
Host | String | XRV_HOST | host | --host
Port | Integer | XRV_PORT | port | --port
Configuration file | String | XRV_CONFIG || --config
Log level| Integer | XRV_LOG_LEVEL | loglevel | -v --verbose -q --quiet
Log in json | Boolean | XRV_LOG_JSON | json_logs | --json-logging --line-logging

## Todo

* Provide a bulk test_case_class_suite_from_test_case.
* Provide more backend functions to query the database.
* Build a nice front end to display the data better.

## API Test commands

Get all projects

    curl -X GET http://127.0.0.1:8080/v1/project/all

Get all run_identifer for project

    curl -X GET "http://127.0.0.1:8080/v1/run_identifer?project_sk=4356e44a-805f-4b70-a5bb-d3883bdf6d8f"

Get all test_run for run_identifer

    curl -X GET "http://127.0.0.1:8080/v1/test_run?run_identifer_sk=db7d1337-b007-4ed1-a302-9ef0a6704676"

Get all environments for test_run

    curl -X GET "http://127.0.0.1:8080/v1/environment_for_test_run?test_run_sk=8de6df07-5a67-4e2a-8720-9efa91d6e468"

Get all environment details

    curl -X GET "http://127.0.0.1:8080/v1/environment_details?environment_sk=1862ef83-0416-4c5d-92c0-a6d089c854f6"

Get all test_files for test_run

    curl -X GET "http://127.0.0.1:8080/v1/test_file_for_test_run?test_run_sk=8de6df07-5a67-4e2a-8720-9efa91d6e468"

Get all test_case_pass for test_file_run

    curl -X GET "http://127.0.0.1:8080/v1/test_case_pass_from_test_file_run?test_file_run_sk=17317178-c79d-4d22-bb24-ab35787dd55a"

Get all test_case_failure for test_file_run

    curl -X GET "http://127.0.0.1:8080/v1/test_case_failure_from_test_file_run?test_file_run_sk=17317178-c79d-4d22-bb24-ab35787dd55a"

Get all test_case_error for test_file_run

    curl -X GET "http://127.0.0.1:8080/v1/test_case_error_from_test_file_run?test_file_run_sk=17317178-c79d-4d22-bb24-ab35787dd55a"
