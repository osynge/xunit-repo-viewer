
Vue.component('display-run-identifier', {
    props: ['run_identifer_sk'],
    data() {
        return {
            data: {
                'list_test_run': [],
            }
        }
    },
    watch: {
        run_identifer_sk: function (val) {
            this.getTestRunList();
        },
    },
    beforeUpdate() {

    },
    methods: {
        selectProject(e) {
            this.$emit('select-project', e);
        }
    },
    methods: {
        async getTestRunList() {
            const lurl = '/v1/test_run?run_identifer_sk=' + this.run_identifer_sk;
            const res = await fetch(lurl);
            const data = await res.json();
            this.data.list_test_run = data;
        },
    },
    template: `
    <div>
        {{run_identifer_sk}} => {{ data.list_test_run }}
    </div>
    `
});