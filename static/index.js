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
        'list_run_identifer': [],
        'run_identifer_sk': 'xxx',
        'list_test_run': [],
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
    setRunIdentifierSk: function (sk) {
      this.data.run_identifer_sk = sk;
      this.getTestRunList();
    },
    async getTestRunList() {
      const lurl = '/v1/test_run?run_identifer_sk=' + this.data.run_identifer_sk;
      const res = await fetch(lurl);
      const data = await res.json();
      this.data.list_test_run = data;
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
      <project-picker :projects="data.list_project" @select-project="setProjectSk"></project-picker>
      You selected project {{ this.data.project_sk }} {{this.data.project_hn}}.
      list_run_identifer {{ this.data.list_run_identifer }}.
      <run-identifer-picker :run_identifers="data.list_run_identifer" @select-run-identifer="setRunIdentifierSk"></run-identifer-picker>
      list_test_run {{ this.data.list_test_run }}.
    </div>`
});