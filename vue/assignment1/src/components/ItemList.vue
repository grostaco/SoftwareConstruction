<script lang="ts">
import { defineComponent } from 'vue';

interface Form {
  item: String | undefined,
  qty: number | undefined,
}

export default defineComponent({
  data: function(): { items: Form[], form: Form } {
    return {
      items: [],
      form: {
        item: undefined,
        qty: undefined,
      }
    }
  },
  methods: {
    onSubmit: function() {
      console.log(this.form.item);
      this.items.push({item: this.form.item, qty: this.form.qty});
      this.resetForm();
    },

    resetForm() {
      var self = this;
      Object.keys(this.$data.form).forEach(
        function (key, _index) {
          self.form.item = undefined;
          self.form.qty = undefined;
        }
      )
    },

    remove(index: number) {
      if (index == 0) {
        this.items.shift();
      } else {
        this.items.splice(index, 1);
      }
    },

    clear() {
      this.items = []
    }
  }
})
</script>

<template>
  <form @keyup.enter.native="onSubmit" id="formInput">
    <input v-model="form.item" placeholder="Item"/>
    <input v-model="form.qty" placeholder="Quantity"/>
  </form>
  
  <table class="table">
    <tr>
      <td >Item Number</td>
      <td>Name</td>
      <td>Quantity</td>
    </tr>
    <tr v-for="[i, form] in items.entries()">
      <td>{{i}}</td>
      <td>{{form.item}}</td>
      <td>{{form.qty}}</td>
      <td class="btn">
        <button @click="remove(i)">X</button>
      </td>
    </tr>
  </table>
  
  <button @click="clear()">Clear</button>
</template>

<style scoped>
  .table {
    margin-top: 0.5rem;
    border-collapse: collapse;
    border-spacing: 0;
  }

  .table td {
    border: 1px solid gray;
  }

  .table .btn {
    border: 0px solid;
  }

  #formInput {
    display: flex;
    flex-direction: column;
  }
</style>