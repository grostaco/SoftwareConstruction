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
  <nav class="navbar dflex-justify-center">
    <div class="dflex dflex-col">
      <div class="bold font-lg">Software Construction</div>
      <div class="dflex dflex-gap-sm">
        <div>
          Made (not) with Rust!
        </div>
        ·
        <div>
          Assignment 2
        </div>
      </div>
    </div>
    <div class="dflex dflex-row dflex-gap-md">
      <a href="https://grostaco.github.io/SoftwareConstruction">Home</a>
      <a href="https://github.com/grostaco">GitHub</a>
      <a href="https://grostaco.herokuapp.com/">About Me</a>
      <a href="https://github.com/grostaco/SoftwareConstruction">This site's code</a>
    </div>
  </nav>
  <div class="divider"></div>
  <main>
    <MessageEditor @send-event="submit" />
    <MessageFeed :messages="messages"/>
  </main>
  
</template>

<style>
main {
  margin-top: 2.5rem;
  margin-left: 2.5rem;
}
</style>
