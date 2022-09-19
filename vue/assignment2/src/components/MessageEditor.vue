<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: 'MessageEditor',
    data () {
        return {
            message: ""
        }
    },
    methods: {
        submit(message: string) {
            (this.$refs.message as any).value = '';
            this.message = message;
        },
        emit(event: string, message: string) {
            this.$emit(event, message);
        }
    }
})
</script>

<template>
    <textarea ref="message" @change="submit(($event.target! as any).value)" placeholder="Enter your message here"></textarea>
    <button @click="emit('send-event', message)">Submit</button>
</template>