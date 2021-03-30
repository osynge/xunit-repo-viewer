Vue.component('project', {
    props: ['sk', 'human_name', 'id'],
    data() {
        return {
            data: {
                'cart': '',
            }
        }
    },
    methods: {
        addToCart(item) {
            this.data.cart = item;
            this.$emit('select-project', item);
        }
    },
    template: `
    <div>
        <button @click="addToCart(sk)">
        {{ human_name }}
        </button>
    </div>
    `
});


Vue.component('project-picker', {
    props: ['projects'],
    methods: {
        updateCart(e) {
            this.$emit('select-project', e);
        }
    },
    template: `
    <div>
          <li v-for="project in projects">
          <project :sk="project.sk" :human_name="project.human_name" :id="project.identiifier" @select-project="updateCart"></project>
        </li>
    </div>
    `
});