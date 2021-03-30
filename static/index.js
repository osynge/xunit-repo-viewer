

Vue.component('button-counter22', {
  data: function () {
    return {
      count: 22
    }
  },
  template: '<button v-on:click="count++">You clicked me {{ count }} times.</button>'
})


var url = '/v1/project/all';

const app = new Vue({
  el: "#app",
  created() {
    console.log('created called.');
  },
  data() {
    return {
      data: {
        'list_project': [],
        'project_sk': 'xxx',
        'project_hn': '',
        'list_run_identifer': '',
      }
    }
  },
  beforeMount() {
    this.getProjectList();
  },
  methods: {
    setProjectSk: function (sk) {
      this.data.project_sk = sk;
      for (var i = 0, size = this.data.list_project.length; i < size; i++) {
        var item = this.data.list_project[i];
        if (item.sk == sk) {
          this.data.project_hn = item.human_name
        }
      }
      this.getRunIdentifierList();
    },
    async getRunIdentifierList() {
      const lurl = '/v1/run_identifer?project_sk=' + this.data.project_sk;
      const res = await fetch(lurl);
      const data = await res.json();
      this.data.list_run_identifer = data;
    },
    async getProjectList() {
      const lurl = '/v1/project/all';
      const res = await fetch(lurl);
      const data = await res.json();
      this.data.list_project = data;
    },

  },
  template: `<div>
      <button-counter22></button-counter22>
      <button-counter></button-counter>
      <project-picker :projects="data.list_project" @select-project="setProjectSk"></project-picker>
      You selected project {{ this.data.project_sk }} {{this.data.project_hn}}.
      list_run_identifer {{ this.data.list_run_identifer }}.
    </div>`
})