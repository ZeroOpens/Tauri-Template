<template>
  <Index />
</template>

<script setup lang="ts">
  defineOptions({name: 'App'})
  import Index from '@/pages/index.vue'

  // 禁用鼠标右键
  window.addEventListener('contextmenu', (e) => {
    e.preventDefault();
  });


</script>

<style scoped>

</style>
 