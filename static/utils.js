async function queryUrl(url) {
    const res = await fetch(url);
    return res.json();
}

async function queryProjectsList() {
    return queryUrl('/v1/project/all');
};

async function queryRunIdentifierList(sk) {
    return queryUrl('/v1/run_identifer?project_sk=' + sk);
}

async function queryTestRunList(sk) {
    return queryUrl('/v1/test_run?run_identifer_sk=' + sk);
}

async function queryEnvironment(sk) {
    const env = await queryUrl('/v1/environment_for_test_run?test_run_sk=' + sk);
    return env[0];
}

var environment_details_store = {};

async function queryEnvironmentDetails(sk) {
    if (sk in environment_details_store) {
        return environment_details_store[sk];
    }
    environment_details_store[sk] = queryUrl('/v1/environment_details?environment_sk=' + sk);
    return environment_details_store[sk];
};

async function queryTestFileList(sk) {
    return queryUrl('/v1/test_file_for_test_run?test_run_sk=' + sk);
};

async function queryTestCasePassList(sk) {
    return queryUrl('/v1/test_case_pass_from_test_file_run?test_file_run_sk=' + sk);
};

async function queryTestCaseFailList(sk) {
    return queryUrl('/v1/test_case_failure_from_test_file_run?test_file_run_sk=' + sk);
};

async function queryTestCaseErrorList(sk) {
    return queryUrl('/v1/test_case_error_from_test_file_run?test_file_run_sk=' + sk);
};

var test_class_store = {};

async function queryTestClass(sk) {
    if (sk in test_class_store) {
        return test_class_store[sk];
    }
    test_class_store[sk] = queryUrl('/v1/test_case_class_suite_from_test_case?test_case_sk=' + sk);
    return test_class_store[sk];
};

async function get_test_details_for_test_case(test_case) {
    return await Promise.all(test_case.map(item => queryTestClass(item.test_case_sk)));
}

async function get_test_details_for_file(file_list) {
    return await Promise.all(file_list.map(item => get_test_details_for_test_case(item)));
}

async function get_fail_list_for_file(file_list) {
    return await Promise.all(file_list.map(item => queryTestCaseFailList(item.sk)));
}

async function get_error_list_for_file(file_list) {
    return await Promise.all(file_list.map(item => queryTestCaseErrorList(item.sk)));
}

async function get_pass_list_for_file(file_list) {
    return await Promise.all(file_list.map(item => queryTestCasePassList(item.sk)));
}


function count_cases_file(file) {
    const cases_file = file.map(item => item.length);
    return cases_file.reduce((a, b) => a + b, 0)
}

function count_cases_file_list(file_list) {
    const cases_file = file_list.map(item => count_cases_file(item));
    return cases_file.reduce((a, b) => a + b, 0)
}

function gen_output(run, environment, test_file, test_file_case, test_file_case_details) {
    output = {
        'count': count_cases_file_list(test_file_case)
    };
    if (output.count == 0) {
        return output;
    }
    output.run = {};
    for (var run_counter = 0, run_size = run.length; run_counter < run_size; run_counter++) {
        const cases_count = count_cases_file(test_file_case[run_counter]);
        if (cases_count == 0) {
            continue;
        };
        var run_details = {
            'created': run[run_counter].created,
            'environment': environment[run_counter],
            'files': {},
        };
        for (var file_counter = 0, file_list_size = test_file[run_counter].length; file_counter < file_list_size; file_counter++) {
            if (test_file_case[run_counter][file_counter].length == 0) {
                continue;
            }
            var test_file_details = {
                'directory': test_file[run_counter][file_counter].directory,
                'file_name': test_file[run_counter][file_counter].file_name,
                'suite': {},
            };
            var case_list_size = test_file_case_details[run_counter][file_counter].length;
            console.log('<case_list_size>' + JSON.stringify(case_list_size));
            for (var case_counter = 0; case_counter < case_list_size; case_counter++) {

                var suite = test_file_case_details[run_counter][file_counter][case_counter].suite;
                if (!(suite in test_file_details.suite)) {
                    test_file_details.suite[suite] = { 'class': {} };
                };
                var test_class = test_file_case_details[run_counter][file_counter][case_counter].class;
                if (!(test_class in test_file_details.suite[suite].class)) {
                    test_file_details.suite[suite].class[test_class] = {};
                }
                var test_case_output = {};
                for (var test_case_key in test_file_case[run_counter][file_counter][case_counter]) {
                    if (test_case_key == 'test_case_sk') {
                        continue;
                    }
                    test_case_output[test_case_key] = test_file_case[run_counter][file_counter][case_counter][test_case_key];
                }

                var test_case_sk = test_file_case[run_counter][file_counter][case_counter].test_case_sk
                test_file_details.suite[suite].class[test_class][test_case_sk] = test_case_output;
            }
            run_details.files[test_file[run_counter][file_counter].sk] = test_file_details;
        }
        output.run[run[run_counter].sk] = run_details;
    }
    return output;
}

async function get_run_list(run_identifer_sk) {
    const run_list = await queryTestRunList(run_identifer_sk);
    console.log('<run_list>' + JSON.stringify(run_list));
    var environment_key = await Promise.all(run_list.map(item => queryEnvironment(item.sk)));
    var environment_details = await Promise.all(environment_key.map(item => queryEnvironmentDetails(item.sk)));
    var test_file_list = await Promise.all(run_list.map(item => queryTestFileList(item.sk)));
    var test_file_list_pass = await Promise.all(test_file_list.map(item => get_pass_list_for_file(item)));
    var test_file_list_error = await Promise.all(test_file_list.map(item => get_error_list_for_file(item)));
    var test_file_list_fail = await Promise.all(test_file_list.map(item => get_fail_list_for_file(item)));
    var test_file_list_pass_details = await Promise.all(test_file_list_pass.map(item => get_test_details_for_file(item)));
    var test_file_list_error_details = await Promise.all(test_file_list_error.map(item => get_test_details_for_file(item)));
    var test_file_list_fail_details = await Promise.all(test_file_list_fail.map(item => get_test_details_for_file(item)));
    var pass_count = count_cases_file_list(test_file_list_pass);
    var error_count = count_cases_file_list(test_file_list_error);
    var fail_count = count_cases_file_list(test_file_list_fail);
    console.log('<environment_key>' + JSON.stringify(environment_key));
    /*console.log('<test_file_list>' + JSON.stringify(test_file_list));
    console.log('<test_file_list_pass>' + JSON.stringify(test_file_list_pass));
    */
    console.log('<test_file_list_error>' + JSON.stringify(test_file_list_error));
    console.log('<test_file_list_fail>' + JSON.stringify(test_file_list_fail));
    console.log('<test_file_list_pass_details>' + JSON.stringify(test_file_list_pass_details));
    console.log('<test_file_list_fail_details>' + JSON.stringify(test_file_list_fail_details));
    /*
    console.log('<pass_count>' + JSON.stringify(pass_count));
    console.log('<error_count>' + JSON.stringify(error_count));
    console.log('<fail_count>' + JSON.stringify(fail_count));
    */

    return {
        'error': gen_output(run_list, environment_details, test_file_list, test_file_list_error, test_file_list_error_details),
        'fail': gen_output(run_list, environment_details, test_file_list, test_file_list_fail, test_file_list_fail_details),
        'pass': gen_output(run_list, environment_details, test_file_list, test_file_list_pass, test_file_list_pass_details),
    };
}