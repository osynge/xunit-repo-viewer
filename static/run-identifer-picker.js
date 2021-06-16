Vue.component('run-identifer-picker', {
    props: ['run_identifers'], data() {
        return {
            data: {
                'run_identifer_selected': '',
            }
        }
    },
    methods: {
        selectRunIdentifer() {
            this.$emit('select-run-identifer', this.data.run_identifer_selected);
        }
    },
    template: `
    <div>
    <div>
        Run Id:
        <select v-model="data.run_identifer_selected" @change="selectRunIdentifer">
            <option disabled value="">Please select</option>
            <option v-for="run_identifer in run_identifers" v-bind:value="run_identifer.sk">{{run_identifer.client_identifier}}</option>
        </select>
    </div>
    `
});