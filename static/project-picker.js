Vue.component('project', {
    props: ['sk', 'human_name', 'id'],
    methods: {
        selectProject(item) {
            this.$emit('select-project', item);
        }
    },
    template: `
    <div>
        <button @click="selectProject(sk)">
        {{ human_name }}
        </button>
    </div>
    `
});


Vue.component('project-picker', {
    props: ['projects'],
    methods: {
        selectProject(e) {
            this.$emit('select-project', e);
        }
    },
    template: `
    <div>
          <li v-for="project in projects">
          <project :sk="project.sk" :human_name="project.human_name" :id="project.identiifier" @select-project="selectProject"></project>
        </li>
    </div>
    `
});