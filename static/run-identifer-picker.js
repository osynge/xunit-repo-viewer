Vue.component('run-identifer', {
    props: ['sk', 'client_identifier', 'created'],
    methods: {
        selectRunIdentifer(item) {
            this.$emit('select-run-identifer', item);
        }
    },
    template: `
    <div>
        <button @click="selectRunIdentifer(sk)">
        {{ client_identifier }}
        </button>
    </div>
    `
});


Vue.component('run-identifer-picker', {
    props: ['run_identifers'],
    methods: {
        selectRunIdentifer(e) {
            this.$emit('select-run-identifer', e);
        }
    },
    template: `
    <div>
          <li v-for="run_identifer in run_identifers">
          <run-identifer :sk="run_identifer.sk" :client_identifier="run_identifer.client_identifier" :created="run_identifer.created" @select-run-identifer="selectRunIdentifer"></run-identifer>
        </li>
    </div>
    `
});