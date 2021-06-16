
Vue.component('display-test', {
    props: ['results', 'sk'],
    template: `
    <div>
        <h5>display-project-test {{results.name}}</h5>
        <table>
        <tr>
        <td>Key</td>
        <td>Value</td>
        </tr>

        <tr v-for="(value, propertyName) in results">

        <td>{{propertyName}}</td>
        <td><code>{{value}}</code></td>
        </tr>
        </table>
    </div>
    `
});

Vue.component('display-project-class', {
    props: ['results', 'name'],
    template: `
    <div>
        <h4>display-project-class {{name}}</h4>
        <span v-for="(value, propertyName) in results">
        <display-test :results="value" :sk="propertyName"> </display-test>
        </span>
    </div>
    `
});

Vue.component('display-project-suite', {
    props: ['results', 'name'],
    template: `
    <div>
        <h3>display-project-suite {{name}}</h3>
        <span v-for="(value, propertyName) in results.class">
        <display-project-class :results="value" :name="propertyName"> </display-project-class>
        </span>
    </div>
    `
});



Vue.component('display-project-file', {
    props: ['results'],
    template: `
    <div>
        <h2>File: {{results.directory}}/{{results.file_name}}</h2>
        <span v-for="(value, propertyName) in results.suite">
        <display-project-suite :results="value" :name="propertyName"> </display-project-suite>
        </span>
    </div>
    `
});

Vue.component('display-project-run-test-results-run', {
    props: ['results'],
    template: `
    <div>
        <h2>environment</h2>
        <table>
        <tr>
        <td>Key</td>
        <td>Value</td>
        </tr>
        <tr v-for="(value, propertyName) in results.environment">
            <td>{{propertyName}}</td>
            <td>{{value}}</td>
        </tr>
        </table>
        <span v-for="item in results.files">
            <display-project-file :results="item"> </display-project-file>
        </span>
    </div>
    `
});


Vue.component('display-project-run-test-results-type', {
    props: ['results'],
    template: `
    <div>
        <span v-for="item in results.run">
            <display-project-run-test-results-run :results="item"></display-project-run-test-results-run>
        </span>
    </div>
    `
});


Vue.component('display-project-run-test-results', {
    props: ['test_results'],
    template: `
    <div>
        <h1>Test Results</h1>
        <table>
            <tr>
            <td>
            Passes
            </td>
            <td>
            Fails
            </td>
            <td>
            Errors
            </td>
            <td>
            Skipped
            </td>
            <td>
            Total
            </td>
            </tr>
            <tr>
            <td>
            {{test_results.pass.count}}
            </td>
            <td>
            {{test_results.fail.count}}
            </td>
            <td>
            {{test_results.error.count}}
            </td>
            <td>
            {{test_results.skip.count}}
            </td>
            <td>
            {{test_results.fail.count + test_results.pass.count + test_results.error.count + test_results.skip.count}}
            </td>
            </tr>
        </table>
        <div v-if="test_results.fail.count > 0">
        <h1>Failing:</h1>
            <display-project-run-test-results-type :results="test_results.fail"></display-project-run-test-results-type>
        </div>
        <div v-if="test_results.error.count > 0">
        <h1>Errors:</h1>
        <display-project-run-test-results-type :results="test_results.error"></display-project-run-test-results-type>
        </div>
        <div v-if="test_results.skip.count > 0">
        <h1>Skipped:</h1>
        <display-project-run-test-results-type :results="test_results.skip"></display-project-run-test-results-type>
        </div>
        <div v-if="test_results.pass.count > 0">
        <h1>Pass:</h1>
        <display-project-run-test-results-type :results="test_results.pass"></display-project-run-test-results-type>
        </div>
    </div>
    `
});