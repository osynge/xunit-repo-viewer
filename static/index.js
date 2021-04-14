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
        'environment_cache': {},
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
      var get_run_list_out = await get_run_list(this.data.run_identifer_sk);
      console.log('output get_run_list=' + JSON.stringify(get_run_list_out));
      //buildRun(this.data.run_identifer_sk);
      var output = await queryTestRunList(this.data.run_identifer_sk);
      for (var i = 0, size = output.length; i < size; i++) {
        console.log('output[i].sk=' + output[i].sk)
        const sdata = await queryEnvironment(output[i].sk);

        console.log('sdata[0].sk=' + sdata.sk)
        if (!(sdata.sk in this.data.environment_cache)) {
          await this.getEnvironmentDetails(sdata.sk);
        }
        output[i]['environment'] = this.data.environment_cache[sdata.sk];
        output[i]['pi'] = await queryEnvironmentDetails(sdata.sk);
      }
      this.data.list_test_run = output;
    },
    async getEnvironmentDetails(sk) {
      this.data.environment_cache[sk] = await queryEnvironmentDetails(sk);
    },
    async getRunIdentifierList() {
      this.data.list_run_identifer = await queryRunIdentifierList(this.data.project_sk);
    },
    async getProjectList() {
      this.data.list_project = await queryProjectsList();
    },

  },
  template: `<div>
      <project-picker :projects="data.list_project" @select-project="setProjectSk"></project-picker>
      You selected project {{ this.data.project_sk }} {{this.data.project_hn}}.
      list_run_identifer {{ this.data.list_run_identifer }}.
      <run-identifer-picker :run_identifers="data.list_run_identifer" @select-run-identifer="setRunIdentifierSk"></run-identifer-picker>
      list_test_run {{ this.data.list_test_run }}.
      <display-run-identifier :run_identifer_sk="data.run_identifer_sk"></display-run-identifier>
    </div>`
});