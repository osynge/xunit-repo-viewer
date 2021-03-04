Vue.component('bill-component', {
  props: ['friend'],
  template: `
    <div>
    asdasd
      <h4>{{ friend.sk }}</h4>
      <h4>{{ friend.identiifier }}</h4>
      <h4>{{ friend.human_name }}</h4>
      <h4>{{ friend }}</h4>
      <todo-item
      v-for="item in friend"
      v-bind:todo="item"
      v-bind:key="item.id"
    ></todo-item>
    <ol>
    <li v-for="testsuite in friend">
      {{ testsuite.sk }}
      {{ testsuite.identiifier }}
      {{ testsuite.human_name }}
        </li>
        </ol>
    </div>
  `
});

var url = '/v1/project/all';

const app = new Vue({
  el: "#app",
  created() {
    console.log('created called.');
    this.loadAxiosTransactions();
  },
  data() {
    return {
      data: { 'name': {} }
    }
  },
  beforeMount() {
    this.getName();
  },
  methods: {
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
  template: `
    <div>
      <div><bill-component :friend="data.name" /></div>

    </div>

  `
})