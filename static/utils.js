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

async function buildRun(run_identifer_sk) {
    var output = [];
    run_list = await queryTestRunList(run_identifer_sk);
    console.log('<run_list>' + JSON.stringify(run_list));
    for (var i = 0, size = run_list.length; i < size; i++) {
        var run_item = run_list[i];
        console.log('size=' + size)
        console.log('i=' + i)
        console.log('output[i].sk=' + run_list[i].sk)
        const environment = await queryEnvironment(run_list[i].sk);


        console.log('<environment>' + JSON.stringify(environment));
        const environment_details = await queryEnvironmentDetails(environment.sk);
        run_item['environment'] = environment_details;
        console.log('<environment_details>' + JSON.stringify(environment_details));
        const test_file_list = await queryTestFileList(run_list[i].sk);
        console.log('<test_file_list>' + JSON.stringify(test_file_list));
        var run_item_test_file_list = []
        for (var j = 0, test_file_list_size = test_file_list.length; j < test_file_list_size; j++) {
            var test_file = test_file_list[j];
            //console.log('test_file_list[j].sk' + JSON.stringify(test_file_list[j].sk));
            const test_case_pass = await queryTestCasePassList(test_file_list[j].sk);
            //console.log('<test_case_pass>' + JSON.stringify(test_case_pass));
            var test_suites = {};
            for (var k = 0, test_case_pass_list_size = test_case_pass.length; k < test_case_pass_list_size; k++) {
                var test_file_pass_item = test_case_pass[k];
                const test_case_sk = test_case_pass[k].test_case_sk;
                const test_class = await queryTestClass(test_case_sk);
                if (!(test_class.suite in test_suites)) {
                    test_suites[test_class.suite] = {};
                }
                if (!(test_class.class in test_suites[test_class.suite])) {
                    test_suites[test_class.suite][test_class.class] = [];
                }
                test_suites[test_class.suite][test_class.class].push(test_file_pass_item);
            }
            test_file['pass'] = test_suites;
            //console.log('<test_pass>' + JSON.stringify(test_file['test_pass']));
            const test_case_fail = await queryTestCaseFailList(test_file_list[j].sk);

            var test_suites = {};
            for (var k = 0, test_case_fail_list_size = test_case_fail.length; k < test_case_fail_list_size; k++) {
                var test_file_pass_item = test_case_fail[k];
                const test_case_sk = test_case_fail[k].test_case_sk;
                const test_class = await queryTestClass(test_case_sk);
                if (!(test_class.suite in test_suites)) {
                    test_suites[test_class.suite] = {};
                }
                if (!(test_class.class in test_suites[test_class.suite])) {
                    test_suites[test_class.suite][test_class.class] = [];
                }
                test_suites[test_class.suite][test_class.class].push(test_file_pass_item);
            }
            test_file['fail'] = test_suites;



            //console.log('<test_case_fail>' + JSON.stringify(test_case_fail));
            const test_case_error = await queryTestCaseErrorList(test_file_list[j].sk);
            //console.log('<test_case_error>' + JSON.stringify(test_case_error));

            var test_suites = {};
            for (var k = 0, test_case_error_list_size = test_case_error.length; k < test_case_error_list_size; k++) {
                var test_case_error_item = test_case_error[k];
                const test_case_sk = test_case_error[k].test_case_sk;
                const test_class = await queryTestClass(test_case_sk);
                if (!(test_class.suite in test_suites)) {
                    test_suites[test_class.suite] = {};
                }
                if (!(test_class.class in test_suites[test_class.suite])) {
                    test_suites[test_class.suite][test_class.class] = [];
                }
                test_suites[test_class.suite][test_class.class].push(test_case_error_item);
            }
            test_file['error'] = test_suites;


            run_item_test_file_list.push(test_file);
        }
        run_item['files'] = run_item_test_file_list;
        output.push(run_item);
    }
    console.log('<output>' + JSON.stringify(output));
    return output;
}