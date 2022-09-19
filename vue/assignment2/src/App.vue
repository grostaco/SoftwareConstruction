<script lang="ts">
import MessageEditor from './components/MessageEditor.vue';
import MessageFeed from './components/MessageFeed.vue';
import { defineComponent } from 'vue';

export default defineComponent( {
  name: 'App',
  components: {
    MessageEditor,
    MessageFeed
},
  data (): { messages: String[] } {
    return {
      messages: [],
    }
  },
  methods: {
    submit (message: string) {
      this.messages.push(message);
    }
  }
})
</script>

<template>
  <MessageEditor @send-event="submit" />
  <MessageFeed :messages="messages"/>
</template>

<style>

</style>
