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
