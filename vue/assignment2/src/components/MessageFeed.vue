<template>
<p v-for="message in messages" :key="message">{{ message }}</p>
</template>

<script lang="ts">
export default {
    name: 'MessageFeed',
    props: {
        messages: Array,
    },
}
</script>