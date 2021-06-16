Vue.component('project-picker', {
    props: ['projects'],
    data() {
        return {
            data: {
                'project_selected': '',
            }
        }
    },
    methods: {
        selectProject() {
            this.$emit('select-project', this.data.project_selected);
        }
    },
    template: `
    <div>
        Project:
        <select v-model="data.project_selected" @change="selectProject">
            <option disabled value="">Please select</option>
            <option v-for="project in projects" v-bind:value="project.sk">{{project.human_name}}</option>
        </select>
    </div>
    `
});
