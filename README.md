# xunit-repo-viewer

## Introduction

The xunit-repo-viewer servers junit/[xunit2](https://xunit.net/) results from a local database. The [xunit-repo](https://github.com/osynge/xunit-repo) updates this database from results provided by the [xunit-repo-client](https://github.com/osynge/xunit-repo-client).

xunit-repo-viewer can be configured with environment variables, configuration files, or command line arguments, in order from lowest to highest precedence. xunit-repo-viewer is expected to be used either on the developers desktop, or as part of a continuous integration, continuous deployment framework such as jenkins or drone.

## Todo:

* Provide more backend functions to query the database.
* Build a nice front end to display the data.
* Add logging.

## API Test commands:

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
