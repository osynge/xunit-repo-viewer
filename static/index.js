


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
    this.loadAxiosTransactions();
  },
  data() {
    return {
      data: {
        'name': [],
        'project_sk': 'xxx',
        'project_hn': '',
      }
    }
  },
  beforeMount() {
    this.getName();
  },
  methods: {
    setProjectSk: function (sk) {
      this.data.project_sk = sk;
      for (var i = 0, size = this.data.name.length; i < size; i++) {
        var item = this.data.name[i];
        if (item.sk == sk) {
          this.data.project_hn = item.human_name
        }
      }

    },
    async getName() {
      const res = await fetch(url);
      const data = await res.json();
      this.data.name = data;
    },
    loadAxiosTransactions() {
      fetch(url)
        .then(function (response) {
          if (response.status != 200) {
            console.log(response.status);
          } else {
            response.json().then(function (data) {
              this.response = data;
            }.bind(this));
          }
        }.bind(this))
    },


  },
  template: `<div>
      <button-counter22></button-counter22>
      <button-counter></button-counter>
      <project-picker :projects="data.name" @select-project="setProjectSk"></project-picker>
      You selected project {{ this.data.project_sk }} {{this.data.project_hn}}.
    </div>`
})