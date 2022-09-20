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
  <div class="dflex" style="justify-content: center;">
    <form @submit.prevent @keyup.enter.native="onSubmit" class="dflex dflex-row dflex-gap-sm" style="justify-content: center;" id="formInput" >
      <input id="item-input" v-model="form.item" placeholder="Item"/>
      <input id="quantity-input" v-model="form.qty" placeholder="Quantity"/> 
    </form>
    <button style="border-radius: 4px; color: red;" @click="clear()">Clear</button>
  </div>
  
  

  <div class="wrapper">
    <div v-for="[i, form] in items.entries()" class="dflex dflex-col dflex-gap-sm dflex-justify-center" style="border-width: 1px; border-color: white; border-style: solid white;">
      <div>{{form.item}}</div>
      <div>Quantity: {{form.qty}}</div>
      <button @click="remove(i)">Clear</button>
    </div>
  </div>

  <!-- <table class="table">
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
   -->
  <!-- <button @click="clear()">Clear</button> -->
</template>

<style scoped>
  #item-input {
    border-radius: 4px;
    height: 2rem;
  }
  #quantity-input {
    border-radius: 4px;
    height: 2rem;
    width: 20%;
  }

  .wrapper {
    display: grid;
    margin-top: 2rem;
    margin-left: 2rem;
    margin-right: 2rem;
    grid-template-columns: repeat(5, 1fr);
    grid-auto-rows: 100px;
    gap: 20px;
  }

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

</style>